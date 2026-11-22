//! Local-first file intelligence for LMLMxtend.
//!
//! The extension studies directories, proposes or performs conservative local
//! organization, and merges supported UTF-8 text files. Repository organization
//! is always report-only. This is trusted in-process code, not an OS sandbox.

use lmlmxtend_api::{
    CapabilityId, ExecutionContext, Extension, ExtensionError, ExtensionManifest, ExtensionRequest,
    ExtensionResponse, API_VERSION,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, HashSet},
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Cursor, Read, Write},
    path::{Path, PathBuf},
};

const MAX_INPUT_BYTES: u64 = 512 * 1024 * 1024;
const MAX_MERGE_BYTES: u64 = 32 * 1024 * 1024;
const MAX_DOCUMENT_MERGE_BYTES: u64 = 128 * 1024 * 1024;
const HASH_LIMIT_BYTES: u64 = 512 * 1024 * 1024;
const SKIP_DIRS: &[&str] = &[
    ".git",
    ".hg",
    ".svn",
    "node_modules",
    "target",
    "vendor",
    ".venv",
    "venv",
    "__pycache__",
    ".cache",
    "dist",
    "build",
];
const TEXT_EXTENSIONS: &[&str] = &[
    "txt", "md", "markdown", "csv", "tsv", "log", "rst", "json", "yaml", "yml", "toml", "xml",
    "html", "css", "js", "ts", "rs", "py", "go", "java", "c", "h", "cpp", "sh", "sql",
];

#[derive(Clone, Debug, Deserialize)]
pub struct FileOperation {
    pub operation: String,
    pub root: Option<PathBuf>,
    #[serde(default)]
    pub confirm: bool,
    #[serde(default)]
    pub files: Vec<PathBuf>,
    pub output: Option<PathBuf>,
}

#[derive(Clone, Debug, Serialize)]
pub struct FileRecord {
    pub path: String,
    pub category: String,
    pub extension: String,
    pub bytes: u64,
    pub modified_unix_seconds: Option<u64>,
    pub sha256: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct DuplicateGroup {
    pub sha256: String,
    pub files: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct StudyReport {
    pub root: String,
    pub repository: bool,
    pub files_scanned: usize,
    pub total_bytes: u64,
    pub files: Vec<FileRecord>,
    pub duplicate_candidates: Vec<DuplicateGroup>,
    pub skipped_entries: usize,
}

#[derive(Clone, Debug, Serialize)]
pub struct OrganizationAction {
    pub source: String,
    pub destination: String,
    pub applied: bool,
    pub reason: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct OrganizationReport {
    pub root: String,
    pub repository: bool,
    pub dry_run: bool,
    pub actions: Vec<OrganizationAction>,
}

#[derive(Clone, Debug, Serialize)]
pub struct MergeReport {
    pub output: String,
    pub inputs: Vec<String>,
    pub bytes_written: u64,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum OperationReport {
    Study(StudyReport),
    Organize(OrganizationReport),
    Merge(MergeReport),
}

#[derive(Default)]
pub struct FileIntelligence;

impl Extension for FileIntelligence {
    fn manifest(&self) -> ExtensionManifest {
        ExtensionManifest {
            id: "productivity.file-intelligence".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            api_version: API_VERSION,
            capabilities: vec![
                CapabilityId::new("files.study").expect("static capability"),
                CapabilityId::new("files.organize").expect("static capability"),
                CapabilityId::new("files.merge").expect("static capability"),
            ],
        }
    }

    fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse, ExtensionError> {
        self.execute_with_context(request, &ExecutionContext::default())
    }

    fn execute_with_context(
        &self,
        request: ExtensionRequest,
        context: &ExecutionContext,
    ) -> Result<ExtensionResponse, ExtensionError> {
        context.check_cancelled().map_err(execution_error)?;
        let operation: FileOperation = serde_json::from_str(&request.input)
            .map_err(|e| ExtensionError::InvalidInput(format!("expected operation JSON: {e}")))?;
        let expected_capability = if operation.operation == "arrange" {
            "files.organize".to_owned()
        } else {
            format!("files.{}", operation.operation)
        };
        if request.capability.as_str() != expected_capability {
            return Err(ExtensionError::InvalidInput(format!(
                "request capability '{}' does not match operation '{}'",
                request.capability.as_str(),
                operation.operation
            )));
        }

        let report = match operation.operation.as_str() {
            "study" => OperationReport::Study(study(required_root(operation.root)?, context)?),
            "organize" | "arrange" => OperationReport::Organize(organize(
                required_root(operation.root)?,
                operation.confirm,
                context,
            )?),
            "merge" => OperationReport::Merge(merge(
                operation.files,
                operation.output.ok_or_else(|| {
                    ExtensionError::InvalidInput("merge requires an output path".into())
                })?,
                context,
            )?),
            other => {
                return Err(ExtensionError::InvalidInput(format!(
                    "unknown operation '{other}'; use study, organize, or merge"
                )))
            }
        };

        let output = serde_json::to_string_pretty(&report)
            .map_err(|e| ExtensionError::ExecutionFailed(e.to_string()))?;
        if output.len() > context.max_output_bytes() {
            return Err(ExtensionError::ExecutionFailed(
                "report exceeds the configured output limit".into(),
            ));
        }
        Ok(ExtensionResponse {
            output,
            media_type: "application/json; charset=utf-8".into(),
        })
    }
}

fn required_root(root: Option<PathBuf>) -> Result<PathBuf, ExtensionError> {
    root.ok_or_else(|| ExtensionError::InvalidInput("operation requires a root path".into()))
}

fn execution_error(error: impl std::fmt::Display) -> ExtensionError {
    ExtensionError::ExecutionFailed(error.to_string())
}

fn canonical_directory(path: &Path) -> Result<PathBuf, ExtensionError> {
    let canonical = fs::canonicalize(path)
        .map_err(|e| ExtensionError::InvalidInput(format!("cannot open root: {e}")))?;
    if !canonical.is_dir() {
        return Err(ExtensionError::InvalidInput(
            "root must be a directory".into(),
        ));
    }
    Ok(canonical)
}

fn is_repository(root: &Path) -> bool {
    root.ancestors()
        .any(|ancestor| ancestor.join(".git").exists())
}

fn category_for(extension: &str) -> &'static str {
    match extension {
        "pdf" => "PDFs",
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" | "heic" => "Images",
        "mp3" | "wav" | "m4a" | "flac" | "ogg" => "Audio",
        "mp4" | "mov" | "mkv" | "webm" => "Video",
        "doc" | "docx" | "odt" | "rtf" => "Documents",
        "xls" | "xlsx" | "ods" | "csv" => "Spreadsheets",
        "ppt" | "pptx" | "odp" => "Presentations",
        "zip" | "tar" | "gz" | "7z" | "rar" => "Archives",
        "rs" | "go" | "py" | "js" | "ts" | "tsx" | "jsx" | "c" | "h" | "cpp" | "java" | "swift"
        | "sh" | "sql" => "Code",
        "md" | "txt" | "rst" | "log" => "Text",
        _ => "Other",
    }
}

fn extension_of(path: &Path) -> String {
    path.extension()
        .and_then(|v| v.to_str())
        .unwrap_or("no-extension")
        .to_ascii_lowercase()
}

fn should_skip_dir(name: &str) -> bool {
    name.starts_with('.') || SKIP_DIRS.contains(&name)
}

fn hash_file(path: &Path, context: &ExecutionContext) -> Result<String, ExtensionError> {
    let file = File::open(path).map_err(execution_error)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    loop {
        context.check_cancelled().map_err(execution_error)?;
        let count = reader.read(&mut buffer).map_err(execution_error)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn study(root: PathBuf, context: &ExecutionContext) -> Result<StudyReport, ExtensionError> {
    let root = canonical_directory(&root)?;
    let repository = is_repository(&root);
    let mut stack = vec![root.clone()];
    let mut files = Vec::new();
    let mut skipped_entries = 0usize;
    let mut total_bytes = 0u64;
    let mut hashes: BTreeMap<String, Vec<String>> = BTreeMap::new();

    while let Some(directory) = stack.pop() {
        context.check_cancelled().map_err(execution_error)?;
        let entries = fs::read_dir(&directory).map_err(execution_error)?;
        for entry in entries {
            context.check_cancelled().map_err(execution_error)?;
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => {
                    skipped_entries += 1;
                    continue;
                }
            };
            let path = entry.path();
            let metadata = match fs::symlink_metadata(&path) {
                Ok(metadata) => metadata,
                Err(_) => {
                    skipped_entries += 1;
                    continue;
                }
            };
            if metadata.file_type().is_symlink() {
                skipped_entries += 1;
                continue;
            }
            if metadata.is_dir() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if !should_skip_dir(&name) {
                    stack.push(path);
                } else {
                    skipped_entries += 1;
                }
                continue;
            }
            if !metadata.is_file() {
                skipped_entries += 1;
                continue;
            }
            let relative = path.strip_prefix(&root).unwrap_or(&path);
            let extension = extension_of(&path);
            let bytes = metadata.len();
            total_bytes = total_bytes.saturating_add(bytes);
            let digest = if bytes <= HASH_LIMIT_BYTES {
                match hash_file(&path, context) {
                    Ok(hash) => {
                        hashes
                            .entry(hash.clone())
                            .or_default()
                            .push(relative.display().to_string());
                        Some(hash)
                    }
                    Err(_) => {
                        skipped_entries += 1;
                        None
                    }
                }
            } else {
                None
            };
            let modified_unix_seconds = metadata
                .modified()
                .ok()
                .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|duration| duration.as_secs());
            files.push(FileRecord {
                path: relative.display().to_string(),
                category: category_for(&extension).into(),
                extension,
                bytes,
                modified_unix_seconds,
                sha256: digest,
            });
            if total_bytes > MAX_INPUT_BYTES.saturating_mul(1024) {
                return Err(ExtensionError::InvalidInput(
                    "study target exceeds the 512 GiB safety ceiling".into(),
                ));
            }
        }
    }

    files.sort_by(|a, b| a.path.cmp(&b.path));
    let duplicate_candidates = hashes
        .into_iter()
        .filter_map(|(sha256, mut paths)| {
            if paths.len() > 1 {
                paths.sort();
                Some(DuplicateGroup {
                    sha256,
                    files: paths,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(StudyReport {
        root: root.display().to_string(),
        repository,
        files_scanned: files.len(),
        total_bytes,
        files,
        duplicate_candidates,
        skipped_entries,
    })
}

fn unique_destination(directory: &Path, filename: &str) -> PathBuf {
    let original = Path::new(filename);
    let stem = original
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file");
    let extension = original.extension().and_then(|s| s.to_str());
    let mut candidate = directory.join(filename);
    let mut index = 1u32;
    while candidate.exists() {
        let name = match extension {
            Some(ext) => format!("{stem} ({index}).{ext}"),
            None => format!("{stem} ({index})"),
        };
        candidate = directory.join(name);
        index = index.saturating_add(1);
    }
    candidate
}

fn organize(
    root: PathBuf,
    confirm: bool,
    context: &ExecutionContext,
) -> Result<OrganizationReport, ExtensionError> {
    let root = canonical_directory(&root)?;
    let repository = is_repository(&root);
    let dry_run = repository || !confirm;
    let entries = fs::read_dir(&root).map_err(execution_error)?;
    let mut actions = Vec::new();

    for entry in entries {
        context.check_cancelled().map_err(execution_error)?;
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => continue,
        };
        let source = entry.path();
        let metadata = match fs::symlink_metadata(&source) {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            continue;
        }
        let filename = match source.file_name().and_then(|s| s.to_str()) {
            Some(name) => name.to_owned(),
            None => continue,
        };
        let category = category_for(&extension_of(&source));
        let destination_dir = root.join(category);
        let destination = unique_destination(&destination_dir, &filename);

        if repository {
            actions.push(OrganizationAction {
                source: source.display().to_string(),
                destination: destination.display().to_string(),
                applied: false,
                reason: Some("repository mode is plan-only; source files are never moved".into()),
            });
            continue;
        }
        if !confirm {
            actions.push(OrganizationAction {
                source: source.display().to_string(),
                destination: destination.display().to_string(),
                applied: false,
                reason: Some("dry run; set confirm=true to apply local moves".into()),
            });
            continue;
        }
        if destination_dir.exists() {
            let metadata = fs::symlink_metadata(&destination_dir).map_err(execution_error)?;
            if metadata.file_type().is_symlink() || !metadata.is_dir() {
                return Err(ExtensionError::InvalidInput(format!(
                    "refusing unsafe category directory: {}",
                    destination_dir.display()
                )));
            }
        } else {
            fs::create_dir(&destination_dir).map_err(execution_error)?;
        }
        let destination = unique_destination(&destination_dir, &filename);
        fs::rename(&source, &destination).map_err(execution_error)?;
        actions.push(OrganizationAction {
            source: source.display().to_string(),
            destination: destination.display().to_string(),
            applied: true,
            reason: None,
        });
    }

    Ok(OrganizationReport {
        root: root.display().to_string(),
        repository,
        dry_run,
        actions,
    })
}

fn merge(
    inputs: Vec<PathBuf>,
    output: PathBuf,
    context: &ExecutionContext,
) -> Result<MergeReport, ExtensionError> {
    if inputs.len() < 2 {
        return Err(ExtensionError::InvalidInput(
            "merge requires at least two input files".into(),
        ));
    }
    let output_parent = output
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let output_parent = fs::canonicalize(output_parent).map_err(execution_error)?;
    let output_name = output
        .file_name()
        .ok_or_else(|| ExtensionError::InvalidInput("output must include a filename".into()))?;
    let output = output_parent.join(output_name);
    let output_canonical_candidate = output.clone();

    let mut canonical_inputs = Vec::new();
    let mut expected_extension: Option<String> = None;
    let mut total_input_bytes = 0u64;
    for input in &inputs {
        context.check_cancelled().map_err(execution_error)?;
        let metadata = fs::symlink_metadata(input).map_err(execution_error)?;
        if !metadata.is_file() || metadata.file_type().is_symlink() {
            return Err(ExtensionError::InvalidInput(format!(
                "merge input must be a regular, non-symlink file: {}",
                input.display()
            )));
        }
        let canonical = fs::canonicalize(input).map_err(execution_error)?;
        if canonical == output_canonical_candidate {
            return Err(ExtensionError::InvalidInput(
                "output cannot overwrite an input file".into(),
            ));
        }
        let extension = extension_of(&canonical);
        if !TEXT_EXTENSIONS.contains(&extension.as_str())
            && !matches!(extension.as_str(), "pdf" | "docx")
        {
            return Err(ExtensionError::InvalidInput(format!(
                "unsupported merge type: .{extension}; supported formats are text, PDF, and DOCX"
            )));
        }
        if expected_extension.as_ref().is_some_and(|v| v != &extension) {
            return Err(ExtensionError::InvalidInput(
                "all merge inputs must have the same file extension".into(),
            ));
        }
        expected_extension = Some(extension);
        total_input_bytes = total_input_bytes.saturating_add(metadata.len());
        let size_limit = if matches!(extension.as_str(), "pdf" | "docx") {
            MAX_DOCUMENT_MERGE_BYTES
        } else {
            MAX_MERGE_BYTES
        };
        if total_input_bytes > size_limit {
            return Err(ExtensionError::InvalidInput(format!(
                "combined merge input exceeds {} MiB",
                size_limit / (1024 * 1024)
            )));
        }
        canonical_inputs.push(canonical);
    }
    if output.exists() {
        return Err(ExtensionError::InvalidInput(format!(
            "refusing to overwrite existing output: {}",
            output.display()
        )));
    }

    match expected_extension.as_deref() {
        Some("pdf") => return merge_pdf_files(canonical_inputs, output, context),
        Some("docx") => return merge_docx_files(canonical_inputs, output, context),
        _ => {}
    }

    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&output)
        .map_err(execution_error)?;
    let mut writer = BufWriter::new(file);
    let mut bytes_written = 0u64;
    for (index, input) in canonical_inputs.iter().enumerate() {
        context.check_cancelled().map_err(execution_error)?;
        let name = input.file_name().unwrap_or_default().to_string_lossy();
        let separator = format!("\n\n===== FILE {}: {} =====\n", index + 1, name);
        writer
            .write_all(separator.as_bytes())
            .map_err(execution_error)?;
        bytes_written += separator.len() as u64;
        let mut reader = BufReader::new(File::open(input).map_err(execution_error)?);
        let mut line = String::new();
        loop {
            context.check_cancelled().map_err(execution_error)?;
            line.clear();
            let count = reader.read_line(&mut line).map_err(execution_error)?;
            if count == 0 {
                break;
            }
            if line.contains('\0') {
                return Err(ExtensionError::InvalidInput(format!(
                    "input is not UTF-8 text: {}",
                    input.display()
                )));
            }
            writer.write_all(line.as_bytes()).map_err(execution_error)?;
            bytes_written += count as u64;
        }
    }
    writer.flush().map_err(execution_error)?;

    Ok(MergeReport {
        output: output.display().to_string(),
        inputs: canonical_inputs
            .iter()
            .map(|p| p.display().to_string())
            .collect(),
        bytes_written,
    })
}


fn write_new_output(path: &Path, bytes: &[u8]) -> Result<(), ExtensionError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(execution_error)?;
    file.write_all(bytes).map_err(execution_error)?;
    file.flush().map_err(execution_error)?;
    file.sync_all().map_err(execution_error)?;
    Ok(())
}

fn merge_pdf_files(
    inputs: Vec<PathBuf>,
    output: PathBuf,
    context: &ExecutionContext,
) -> Result<MergeReport, ExtensionError> {
    use lopdf::{Dictionary, Document as PdfDocument, Error as PdfError, Object, ObjectId};

    fn inherited_attribute(
        document: &PdfDocument,
        page_id: ObjectId,
        key: &[u8],
    ) -> Result<Option<Object>, PdfError> {
        let mut current = page_id;
        let mut visited = HashSet::new();
        loop {
            if !visited.insert(current) {
                return Err(PdfError::ReferenceCycle(current));
            }
            let node = document.get_dictionary(current)?;
            if let Ok(value) = node.get(key) {
                match document.dereference(value) {
                    Ok((_, Object::Null)) | Err(PdfError::ObjectNotFound(_)) => {}
                    Ok(_) => return Ok(Some(value.clone())),
                    Err(error) => return Err(error),
                }
            }
            match node.get(b"Parent") {
                Err(_) | Ok(Object::Null) => return Ok(None),
                Ok(parent) => current = parent.as_reference()?,
            }
        }
    }

    fn materialize_page(
        document: &PdfDocument,
        page_id: ObjectId,
    ) -> Result<Dictionary, PdfError> {
        let mut page = document.get_dictionary(page_id)?.clone();
        let media_box = inherited_attribute(document, page_id, b"MediaBox")?
            .ok_or_else(|| PdfError::DictKey("MediaBox".to_owned()))?;
        let crop_box =
            inherited_attribute(document, page_id, b"CropBox")?.unwrap_or_else(|| media_box.clone());
        let resources = inherited_attribute(document, page_id, b"Resources")?
            .unwrap_or_else(|| Object::Dictionary(Dictionary::new()));
        let rotation =
            inherited_attribute(document, page_id, b"Rotate")?.unwrap_or(Object::Integer(0));
        page.set("MediaBox", media_box);
        page.set("CropBox", crop_box);
        page.set("Resources", resources);
        page.set("Rotate", rotation);
        Ok(page)
    }

    let mut merged = PdfDocument::with_version("1.5");
    let mut next_object_id = 1u32;
    let mut page_order: Vec<ObjectId> = Vec::new();
    let mut page_objects = BTreeMap::new();
    let mut first_catalog: Option<(ObjectId, Dictionary)> = None;
    let mut first_pages: Option<(ObjectId, Dictionary)> = None;

    for input in &inputs {
        context.check_cancelled().map_err(execution_error)?;
        let mut source = PdfDocument::load(input).map_err(execution_error)?;
        if source.is_encrypted() {
            return Err(ExtensionError::InvalidInput(format!(
                "encrypted PDFs are not supported: {}",
                input.display()
            )));
        }

        let original_pages = source.get_pages();
        if original_pages.is_empty() {
            return Err(ExtensionError::InvalidInput(format!(
                "PDF contains no pages: {}",
                input.display()
            )));
        }

        for page_id in original_pages.values() {
            let page = materialize_page(&source, *page_id).map_err(execution_error)?;
            source.objects.insert(*page_id, Object::Dictionary(page));
        }

        source.renumber_objects_with(next_object_id);
        next_object_id = source.max_id.saturating_add(1);

        let catalog_id = source
            .trailer
            .get(b"Root")
            .map_err(execution_error)?
            .as_reference()
            .map_err(execution_error)?;
        let catalog = source
            .get_dictionary(catalog_id)
            .map_err(execution_error)?
            .clone();
        let pages_id = catalog
            .get(b"Pages")
            .map_err(execution_error)?
            .as_reference()
            .map_err(execution_error)?;
        let pages = source
            .get_dictionary(pages_id)
            .map_err(execution_error)?
            .clone();

        if first_catalog.is_none() {
            first_catalog = Some((catalog_id, catalog));
            first_pages = Some((pages_id, pages));
        }

        for page_id in source.get_pages().values() {
            page_order.push(*page_id);
            let page = source.get_object(*page_id).map_err(execution_error)?.clone();
            page_objects.insert(*page_id, page);
        }

        for (object_id, object) in source.objects {
            match object.type_name().unwrap_or(b"") {
                b"Catalog" | b"Pages" | b"Page" | b"Outlines" | b"Outline" => {}
                _ => {
                    merged.objects.insert(object_id, object);
                }
            }
        }
    }

    merged.objects.extend(page_objects);
    let (catalog_id, mut catalog) = first_catalog
        .ok_or_else(|| ExtensionError::InvalidInput("no PDF catalog found".into()))?;
    let (pages_id, mut pages) = first_pages
        .ok_or_else(|| ExtensionError::InvalidInput("no PDF page tree found".into()))?;

    for page_id in &page_order {
        let page = merged
            .objects
            .get_mut(page_id)
            .ok_or_else(|| ExtensionError::ExecutionFailed("merged PDF page object is missing".into()))?;
        let dictionary = page.as_dict_mut().map_err(execution_error)?;
        dictionary.set("Parent", Object::Reference(pages_id));
    }

    pages.remove(b"Parent");
    for key in [b"MediaBox".as_slice(), b"CropBox", b"Resources", b"Rotate"] {
        pages.remove(key);
    }
    pages.set("Count", page_order.len() as u32);
    pages.set(
        "Kids",
        page_order.iter().copied().map(Object::Reference).collect::<Vec<_>>(),
    );

    catalog.set("Pages", Object::Reference(pages_id));
    catalog.remove(b"Outlines");

    merged.objects.insert(pages_id, Object::Dictionary(pages));
    merged.objects.insert(catalog_id, Object::Dictionary(catalog));
    merged.trailer.set("Root", Object::Reference(catalog_id));
    merged.max_id = merged.objects.keys().map(|(id, _)| *id).max().unwrap_or(0);

    let mut bytes = Vec::new();
    merged.save_to(&mut bytes).map_err(execution_error)?;
    write_new_output(&output, &bytes)?;

    Ok(MergeReport {
        output: output.display().to_string(),
        inputs: inputs.iter().map(|p| p.display().to_string()).collect(),
        bytes_written: bytes.len() as u64,
    })
}

fn merge_docx_files(
    inputs: Vec<PathBuf>,
    output: PathBuf,
    context: &ExecutionContext,
) -> Result<MergeReport, ExtensionError> {
    use docx_rs::{read_docx, DocumentChild, Docx, Paragraph, Run};

    fn validate_appended_docx(document: &Docx, path: &Path) -> Result<(), ExtensionError> {
        let relationships = &document.document_rels;
        let has_external_parts = relationships.has_comments
            || relationships.has_numberings
            || relationships.has_footnotes
            || relationships.header_count > 0
            || relationships.footer_count > 0
            || relationships.custom_xml_count > 0
            || !relationships.images.is_empty()
            || !relationships.hyperlinks.is_empty()
            || !document.media.is_empty()
            || !document.images.is_empty()
            || !document.hyperlinks.is_empty();

        if has_external_parts {
            return Err(ExtensionError::InvalidInput(format!(
                "DOCX input '{}' contains media, links, numbering, comments, footnotes, headers, footers, or other linked parts; only self-contained body content can be appended in this version",
                path.display()
            )));
        }

        if document
            .document
            .children
            .iter()
            .any(|child| !matches!(child, DocumentChild::Paragraph(_) | DocumentChild::Table(_)))
        {
            return Err(ExtensionError::InvalidInput(format!(
                "DOCX input '{}' contains body elements that require cross-document relationship remapping",
                path.display()
            )));
        }
        Ok(())
    }

    let mut merged: Option<Docx> = None;
    for (index, input) in inputs.iter().enumerate() {
        context.check_cancelled().map_err(execution_error)?;
        let bytes = fs::read(input).map_err(execution_error)?;
        let mut current = read_docx(&bytes).map_err(execution_error)?;

        if index == 0 {
            merged = Some(current);
            continue;
        }

        validate_appended_docx(&current, input)?;
        let base = merged.as_mut().expect("first DOCX was loaded");

        for style in current.styles.styles.drain(..) {
            if let Some(existing) = base
                .styles
                .styles
                .iter()
                .find(|item| item.style_id == style.style_id)
            {
                if existing != &style {
                    return Err(ExtensionError::InvalidInput(format!(
                        "DOCX style '{}' conflicts between source documents",
                        style.style_id
                    )));
                }
            } else {
                base.styles.styles.push(style);
            }
        }

        let source_name = input
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("document");
        base.document
            .children
            .push(DocumentChild::Paragraph(Box::new(
                Paragraph::new().add_run(Run::new().add_text(format!(
                    "— Source document {}: {} —",
                    index + 1,
                    source_name
                ))),
            )));
        base.document
            .children
            .append(&mut current.document.children);
    }

    let merged =
        merged.ok_or_else(|| ExtensionError::InvalidInput("no DOCX inputs supplied".into()))?;
    let mut cursor = Cursor::new(Vec::new());
    merged.pack(&mut cursor).map_err(execution_error)?;
    let bytes = cursor.into_inner();
    write_new_output(&output, &bytes)?;

    Ok(MergeReport {
        output: output.display().to_string(),
        inputs: inputs.iter().map(|p| p.display().to_string()).collect(),
        bytes_written: bytes.len() as u64,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn request(operation: &str, root: &Path, confirm: bool) -> ExtensionRequest {
        let payload = serde_json::json!({
            "operation": operation,
            "root": root,
            "confirm": confirm
        });
        ExtensionRequest::new(
            CapabilityId::new(if operation == "arrange" {
                "files.organize".to_owned()
            } else {
                format!("files.{operation}")
            })
            .unwrap(),
            payload.to_string(),
        )
    }

    #[test]
    fn extension_returns_a_study_report_through_the_public_contract() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("readme.md"), "hello").unwrap();
        let response = FileIntelligence
            .execute(request("study", dir.path(), false))
            .unwrap();
        assert_eq!(response.media_type, "application/json; charset=utf-8");
        let report: serde_json::Value = serde_json::from_str(&response.output).unwrap();
        assert_eq!(report["status"], "study");
        assert_eq!(report["files_scanned"], 1);
    }

    #[test]
    fn rejects_operation_when_capability_does_not_match() {
        let dir = tempdir().unwrap();
        let input = serde_json::json!({
            "operation": "organize",
            "root": dir.path(),
            "confirm": false
        })
        .to_string();
        let request = ExtensionRequest::new(CapabilityId::new("files.study").unwrap(), input);
        assert!(FileIntelligence.execute(request).is_err());
    }

    #[test]
    fn arrange_is_an_alias_for_safe_organization() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("notes.txt"), "notes").unwrap();
        let response = FileIntelligence
            .execute(request("arrange", dir.path(), false))
            .unwrap();
        let report: serde_json::Value = serde_json::from_str(&response.output).unwrap();
        assert_eq!(report["status"], "organize");
        assert_eq!(report["dry_run"], true);
    }

    #[test]
    fn manifest_declares_file_capabilities() {
        let manifest = FileIntelligence.manifest();
        assert_eq!(manifest.id, "productivity.file-intelligence");
        assert_eq!(manifest.capabilities.len(), 3);
    }

    #[test]
    fn study_finds_files_and_duplicate_content() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("a.txt"), "same").unwrap();
        fs::write(dir.path().join("b.txt"), "same").unwrap();
        fs::create_dir(dir.path().join("node_modules")).unwrap();
        fs::write(dir.path().join("node_modules/ignored.txt"), "ignored").unwrap();

        let report = study(dir.path().to_path_buf(), &ExecutionContext::default()).unwrap();
        assert_eq!(report.files_scanned, 2);
        assert_eq!(report.duplicate_candidates.len(), 1);
        assert_eq!(report.duplicate_candidates[0].files.len(), 2);
    }

    #[test]
    fn organize_defaults_to_dry_run_and_never_moves_repository_files() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("notes.txt");
        fs::write(&file, "notes").unwrap();
        let dry = organize(
            dir.path().to_path_buf(),
            false,
            &ExecutionContext::default(),
        )
        .unwrap();
        assert!(dry.dry_run);
        assert!(file.exists());

        fs::create_dir(dir.path().join(".git")).unwrap();
        let repo_plan =
            organize(dir.path().to_path_buf(), true, &ExecutionContext::default()).unwrap();
        assert!(repo_plan.repository);
        assert!(repo_plan.dry_run);
        assert!(file.exists());
    }

    #[test]
    fn local_organize_moves_only_top_level_regular_files_when_confirmed() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("notes.txt");
        fs::write(&file, "notes").unwrap();
        let report =
            organize(dir.path().to_path_buf(), true, &ExecutionContext::default()).unwrap();
        assert!(!report.dry_run);
        assert!(!file.exists());
        assert!(dir.path().join("Text/notes.txt").exists());
    }

    #[test]
    fn merge_creates_new_text_output_without_overwriting_sources() {
        let dir = tempdir().unwrap();
        let a = dir.path().join("a.md");
        let b = dir.path().join("b.md");
        let out = dir.path().join("combined.md");
        fs::write(&a, "alpha").unwrap();
        fs::write(&b, "beta").unwrap();
        merge(
            vec![a.clone(), b.clone()],
            out.clone(),
            &ExecutionContext::default(),
        )
        .unwrap();
        let content = fs::read_to_string(out).unwrap();
        assert!(content.contains("alpha"));
        assert!(content.contains("beta"));
        assert!(a.exists() && b.exists());
    }

    #[test]
    fn rejects_merge_overwrite() {
        let dir = tempdir().unwrap();
        let a = dir.path().join("a.txt");
        let b = dir.path().join("b.txt");
        fs::write(&a, "a").unwrap();
        fs::write(&b, "b").unwrap();
        assert!(merge(vec![a, b.clone()], b, &ExecutionContext::default()).is_err());
    }

    fn minimal_test_pdf() -> Vec<u8> {
        use lopdf::{Dictionary, Document as PdfDocument, Object, Stream};

        let mut document = PdfDocument::with_version("1.5");
        let pages_id = document.new_object_id();
        let content_id = document.add_object(Stream::new(Dictionary::new(), Vec::new()));
        let page_id = document.add_object(lopdf::dictionary! {
            "Type" => "Page",
            "Parent" => pages_id,
            "Contents" => content_id,
            "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
        });
        document.objects.insert(
            pages_id,
            Object::Dictionary(lopdf::dictionary! {
                "Type" => "Pages",
                "Kids" => vec![page_id.into()],
                "Count" => 1,
            }),
        );
        let catalog_id = document.add_object(lopdf::dictionary! {
            "Type" => "Catalog",
            "Pages" => pages_id,
        });
        document.trailer.set("Root", catalog_id);
        let mut bytes = Vec::new();
        document.save_to(&mut bytes).unwrap();
        bytes
    }

    #[test]
    fn pdf_merge_combines_page_trees_and_preserves_source_files() {
        let dir = tempdir().unwrap();
        let first = dir.path().join("first.pdf");
        let second = dir.path().join("second.pdf");
        let output = dir.path().join("combined.pdf");
        fs::write(&first, minimal_test_pdf()).unwrap();
        fs::write(&second, minimal_test_pdf()).unwrap();

        merge(
            vec![first.clone(), second.clone()],
            output.clone(),
            &ExecutionContext::default(),
        )
        .unwrap();

        let merged = lopdf::Document::load(output).unwrap();
        assert_eq!(merged.get_pages().len(), 2);
        assert!(first.exists() && second.exists());
    }

    fn write_test_docx(path: &Path, text: &str) {
        let document = docx_rs::Document::new()
            .add_paragraph(docx_rs::Paragraph::new().add_run(docx_rs::Run::new().add_text(text)));
        docx_rs::Docx::new()
            .document(document)
            .pack(File::create(path).unwrap())
            .unwrap();
    }

    #[test]
    fn docx_merge_appends_body_content_and_preserves_inputs() {
        let dir = tempdir().unwrap();
        let first = dir.path().join("first.docx");
        let second = dir.path().join("second.docx");
        let output = dir.path().join("combined.docx");
        write_test_docx(&first, "first document");
        write_test_docx(&second, "second document");

        merge(
            vec![first.clone(), second.clone()],
            output.clone(),
            &ExecutionContext::default(),
        )
        .unwrap();

        let bytes = fs::read(output).unwrap();
        let merged = docx_rs::read_docx(&bytes).unwrap();
        assert_eq!(merged.document.children.len(), 3);
        assert!(first.exists() && second.exists());
    }
}