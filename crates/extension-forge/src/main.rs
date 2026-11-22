use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

const CATEGORIES: &[&str] = &[
    "creative", "language", "vision", "audio", "spatial", "agents", "productivity",
];

fn main() {
    if let Err(error) = run() {
        eprintln!("lmlmxtend-forge: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("new") => create_extension(args.collect()),
        Some("help") | Some("--help") | Some("-h") | None => {
            print_help();
            Ok(())
        }
        Some(command) => Err(format!(
            "unknown command '{command}'. Run 'lmlmxtend-forge help'."
        )),
    }
}

fn print_help() {
    println!("LMLMxtend Extension Forge

USAGE:
  lmlmxtend-forge new --name <slug> --category <category> --capability <namespace.action> [--root <path>]

CATEGORIES:
  creative, language, vision, audio, spatial, agents, productivity

EXAMPLE:
  lmlmxtend-forge new --name scene-composer --category creative --capability creative.compose

The generator never overwrites existing files.");
}

fn create_extension(args: Vec<String>) -> Result<(), String> {
    let mut name = None;
    let mut category = None;
    let mut capability = None;
    let mut root = PathBuf::from("extensions");
    let mut iter = args.into_iter();
    while let Some(flag) = iter.next() {
        let value = iter
            .next()
            .ok_or_else(|| format!("missing value for {flag}"))?;
        match flag.as_str() {
            "--name" => name = Some(value),
            "--category" => category = Some(value),
            "--capability" => capability = Some(value),
            "--root" => root = PathBuf::from(value),
            _ => return Err(format!("unknown option '{flag}'")),
        }
    }

    let name = name.ok_or("required option: --name")?;
    let category = category.ok_or("required option: --category")?;
    let capability = capability.ok_or("required option: --capability")?;
    validate_slug(&name)?;
    if root.is_absolute()
        || root
            .components()
            .any(|part| matches!(part, Component::ParentDir))
    {
        return Err("--root must be a repository-relative path without '..' components".into());
    }
    if !CATEGORIES.contains(&category.as_str()) {
        return Err(format!(
            "unsupported category '{category}'. Choose: {}",
            CATEGORIES.join(", ")
        ));
    }
    validate_capability(&capability)?;

    let extension_id = format!("{category}.{name}");
    let destination = root.join(&category).join(&name);
    if destination.exists() {
        return Err(format!(
            "{} already exists; refusing to overwrite it",
            destination.display()
        ));
    }
    let api_path = relative_api_path(&destination);
    let manifest = format!(
        "id = \"{extension_id}\"\nname = \"{name}\"\ncategory = \"{category}\"\nversion = \"0.1.0\"\napi_version = 1\ncapabilities = [\"{capability}\"]\n"
    );
    let cargo = format!(
        "[package]\nname = \"lmlmxtend-{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\nlicense = \"MIT\"\ndescription = \"LMLMxtend {category} extension: {name}\"\n\n[dependencies]\nlmlmxtend-api = {{ path = \"{api_path}\" }}\n\n[workspace]\n"
    );
    let source = format!(
        r#"//! Generated LMLMxtend extension. Replace the example behavior with your implementation.

use lmlmxtend_api::{{
    CapabilityId, Extension, ExtensionError, ExtensionManifest, ExtensionRequest,
    ExtensionResponse, API_VERSION,
}};

pub struct {type_name};

impl Extension for {type_name} {{
    fn manifest(&self) -> ExtensionManifest {{
        ExtensionManifest {{
            id: "{extension_id}".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            api_version: API_VERSION,
            capabilities: vec![
                CapabilityId::new("{capability}").expect("generator validates capability IDs"),
            ],
        }}
    }}

    fn execute(&self, request: ExtensionRequest) -> Result<ExtensionResponse, ExtensionError> {{
        // TODO: implement the {category} capability.
        // Treat request.input as untrusted and validate it before processing.
        Ok(ExtensionResponse::text(format!(
            "{name} received {{}} bytes",
            request.input.len()
        )))
    }}
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn exposes_expected_manifest() {{
        let manifest = {type_name}.manifest();
        assert_eq!(manifest.id, "{extension_id}");
        assert_eq!(manifest.api_version, API_VERSION);
        assert_eq!(manifest.capabilities[0].as_str(), "{capability}");
    }}

    #[test]
    fn returns_a_response() {{
        let request = ExtensionRequest::new(
            CapabilityId::new("{capability}").unwrap(),
            "hello",
        );
        let response = {type_name}.execute(request).unwrap();
        assert!(response.output.contains("5 bytes"));
    }}
}}
"#,
        type_name = to_type_name(&name)
    );

    fs::create_dir_all(destination.join("src")).map_err(|e| e.to_string())?;
    write_new(&destination.join("Cargo.toml"), &cargo)?;
    write_new(&destination.join("extension.toml"), &manifest)?;
    write_new(&destination.join("src/lib.rs"), &source)?;
    let readme = format!(
        "# {name}\n\nGenerated LMLMxtend {category} extension.\n\nExtension ID: {extension_id}\nCapability: {capability}\nAPI version: 1\n\nRun cargo test from this directory.\n\nImplement input validation and cooperative cancellation for long-running work.\n"
    );
    write_new(&destination.join("README.md"), &readme)?;
    println!("Created extension: {}", destination.display());
    println!("Extension ID: {extension_id}");
    println!("Capability: {capability}");
    println!(
        "Next: cargo test --manifest-path {}/Cargo.toml",
        destination.display()
    );
    Ok(())
}

fn write_new(path: &Path, content: &str) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| format!("cannot create {}: {e}", path.display()))?;
    file.write_all(content.as_bytes())
        .map_err(|e| e.to_string())
}

fn validate_slug(value: &str) -> Result<(), String> {
    let valid = !value.is_empty()
        && value.len() <= 48
        && value.split('-').all(|part| {
            !part.is_empty()
                && part.as_bytes()[0].is_ascii_lowercase()
                && part
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        });
    if valid {
        Ok(())
    } else {
        Err("name must be 1-48 lowercase ASCII letters, digits, or hyphens; it must start with a letter".into())
    }
}

fn validate_capability(value: &str) -> Result<(), String> {
    lmlmxtend_api::CapabilityId::new(value.to_string())
        .map(|_| ())
        .map_err(|error| error.to_string())
}

fn to_type_name(value: &str) -> String {
    value
        .split('-')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

fn relative_api_path(destination: &Path) -> String {
    let depth = destination.components().count();
    let up = (0..depth).map(|_| "..").collect::<Vec<_>>().join("/");
    format!("{up}/crates/lmlmxtend-api")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unsafe_names() {
        assert!(validate_slug("../outside").is_err());
        assert!(validate_slug("UpperCase").is_err());
        assert!(validate_slug("valid-extension2").is_ok());
        assert!(validate_slug("valid-2extension").is_err());
        assert!(validate_slug("valid--extension").is_err());
    }

    #[test]
    fn validates_capabilities_with_the_api_contract() {
        assert!(validate_capability("creative.compose").is_ok());
        assert!(validate_capability("Creative..compose").is_err());
    }

    #[test]
    fn converts_slug_to_rust_type_name() {
        assert_eq!(to_type_name("scene-composer"), "SceneComposer");
    }

    #[test]
    fn computes_api_path_from_default_extension_root() {
        assert_eq!(
            relative_api_path(Path::new("extensions/creative/scene-composer")),
            "../../../crates/lmlmxtend-api"
        );
    }
}
