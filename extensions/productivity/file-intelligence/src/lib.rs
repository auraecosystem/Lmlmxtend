//! Local-first file intelligence for LMLMxtend.
//!
//! The extension studies directories, proposes or performs conservative local
//! organization, and merges supported UTF-8 text files. Repository organization
//! is always report-only. This is trusted in-process code, not an OS sandbox.

use lmlmxtend_api::{
    CapabilityId, ExecutionContext, Extension, ExtensionError, ExtensionManifest,
    ExtensionRequest, ExtensionResponse, API_VERSION,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

const MAX_INPUT_BYTES: u64 = 512 * 1024 * 1024;
const MAX_MERGE_BYTES: u64 = 32 * 1024 * 1024;
const HASH_LIMIT_BYTES: u64 = 512 * 1024 * 1024;
const SKIP_DIRS: &[&str] = &[
    ".git", ".hg", ".svn", "node_modules", "target", "vendor",
    ".venv", "venv", "__pycache__", ".cache", "dist", "build",
];
const TEXT_EXTENSIONS: &[&str] = &[
    "txt", "md", "markdown", "csv", "tsv", "log", "rst", "json",
    "yaml", "yml", "toml", "xml", "html", "css", "js", "ts", "rs",
    "py", "go", "java", "c", "h", "cpp", "sh", "sql",
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
        let expected_capability = format!("files.{}", operation.operation);
        if request.capability.as_str() != expected_capability {
            return Err(ExtensionError::InvalidInput(format!(
                "request capability '{}' does not match operation '{}'",
                request.capability.as_str(),
                operation.operation
            )));
        }

        let report = match operation.operation.as_str() {
            "study" => OperationReport::Study(study(
                required_root(operation.root)?,
                context,
            )?),
            "organize" => OperationReport::Organize(organize(
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
        return Err(ExtensionError::InvalidInput("root must be a directory".into()));
    }
    Ok(canonical)
}

fn is_repository(root: &Path) -> bool {
    root.ancestors().any(|ancestor| ancestor.join(".git").exists())
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
        "rs" | "go" | "py" | "js" | "ts" | "tsx" | "jsx" | "c" | "h"
        | "cpp" | "java" | "swift" | "sh" | "sql" => "Code",
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
                        hashes.entry(hash.clone()).or_default()
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
            let modified_unix_seconds = metadata.modified().ok()
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
    let duplicate_candidates = hashes.into_iter()
        .filter_map(|(sha256, mut paths)| {
            if paths.len() > 1 {
                paths.sort();
                Some(DuplicateGroup { sha256, files: paths })
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
    let stem = original.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
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
    let output_parent = output.parent().filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let output_parent = fs::canonicalize(output_parent).map_err(execution_error)?;
    let output_name = output.file_name().ok_or_else(|| {
        ExtensionError::InvalidInput("output must include a filename".into())
    })?;
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
        if !TEXT_EXTENSIONS.contains(&extension.as_str()) {
            return Err(ExtensionError::InvalidInput(format!(
                "unsupported text merge type: .{extension}"
            )));
        }
        if expected_extension.as_ref().is_some_and(|v| v != &extension) {
            return Err(ExtensionError::InvalidInput(
                "all merge inputs must have the same file extension".into(),
            ));
        }
        expected_extension = Some(extension);
        total_input_bytes = total_input_bytes.saturating_add(metadata.len());
        if total_input_bytes > MAX_MERGE_BYTES {
            return Err(ExtensionError::InvalidInput(
                "combined merge input exceeds 32 MiB".into(),
            ));
        }
        canonical_inputs.push(canonical);
    }
    if output.exists() {
        return Err(ExtensionError::InvalidInput(format!(
            "refusing to overwrite existing output: {}",
            output.display()
        )));
    }

    let file = OpenOptions::new().write(true).create_new(true)
        .open(&output).map_err(execution_error)?;
    let mut writer = BufWriter::new(file);
    let mut bytes_written = 0u64;
    for (index, input) in canonical_inputs.iter().enumerate() {
        context.check_cancelled().map_err(execution_error)?;
        let name = input.file_name().unwrap_or_default().to_string_lossy();
        let separator = format!(
            "\n\n===== FILE {}: {} =====\n",
            index + 1,
            name
        );
        writer.write_all(separator.as_bytes()).map_err(execution_error)?;
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
        inputs: canonical_inputs.iter().map(|p| p.display().to_string()).collect(),
        bytes_written,
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
            CapabilityId::new(&format!("files.{operation}")).unwrap(),
            payload.to_string(),
        )
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
        let dry = organize(dir.path().to_path_buf(), false, &ExecutionContext::default()).unwrap();
        assert!(dry.dry_run);
        assert!(file.exists());

        fs::create_dir(dir.path().join(".git")).unwrap();
        let repo_plan = organize(dir.path().to_path_buf(), true, &ExecutionContext::default()).unwrap();
        assert!(repo_plan.repository);
        assert!(repo_plan.dry_run);
        assert!(file.exists());
    }

    #[test]
    fn local_organize_moves_only_top_level_regular_files_when_confirmed() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("notes.txt");
        fs::write(&file, "notes").unwrap();
        let report = organize(dir.path().to_path_buf(), true, &ExecutionContext::default()).unwrap();
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
        merge(vec![a.clone(), b.clone()], out.clone(), &ExecutionContext::default()).unwrap();
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
}
