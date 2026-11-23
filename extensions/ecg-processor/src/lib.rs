use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Enforces Lmlmxtend's strict 8 MiB (8,388,608 bytes) input cap before execution
const MAX_PAYLOAD_BYTES: usize = 8 * 1024 * 1024;
const CAPABILITY_ID: &str = "signal.ecg.analyze";
const CONTRACT_VERSION: &str = "v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcgAnalysisRequest {
    pub file_path: String,
    #[serde(default)]
    pub apply_filter: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcgVitals {
    pub r_peaks: Vec<usize>,
    pub bpm: f64,
    pub rr_std_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcgAnalysisResponse {
    pub status: String,
    pub capability: String,
    pub contract_version: String,
    pub num_leads: usize,
    pub samples: usize,
    pub vitals: Option<EcgVitals>,
    pub error: Option<String>,
}

pub struct EcgProcessorExtension {
    script_path: PathBuf,
    python_bin: String,
}

impl Default for EcgProcessorExtension {
    fn default() -> Self {
        Self {
            script_path: PathBuf::from("extensions/ecg-processor/ecg_preprocessing.py"),
            python_bin: "python3".to_string(),
        }
    }
}

impl EcgProcessorExtension {
    pub fn new(script_path: impl Into<PathBuf>, python_bin: impl Into<String>) -> Self {
        Self {
            script_path: script_path.into(),
            python_bin: python_bin.into(),
        }
    }

    /// Advertises capability metadata to lmlmxtend-runtime discovery
    pub fn capability_id(&self) -> &'static str {
        CAPABILITY_ID
    }

    pub fn contract_version(&self) -> &'static str {
        CONTRACT_VERSION
    }

    /// Policy-checked capability execution entrypoint called by lmlmxtend-runtime
    pub fn execute(&self, input: Value) -> Result<Value, String> {
        let mut payload = input;
        if let Some(obj) = payload.as_object_mut() {
            obj.insert("capability".to_string(), json!(CAPABILITY_ID));
        }

        let serialized = serde_json::to_string(&payload)
            .map_err(|e| format!("Failed to serialize capability payload: {e}"))?;

        if serialized.len() > MAX_PAYLOAD_BYTES {
            return Err(format!(
                "Payload size ({} bytes) exceeds LMLMxtend 8 MiB limit",
                serialized.len()
            ));
        }

        if !self.script_path.exists() {
            return Err(format!(
                "Python processor script missing at {}",
                self.script_path.display()
            ));
        }

        // Spawn Python worker with piped stdin/stdout (no network exposure)
        let mut child = Command::new(&self.python_bin)
            .arg(&self.script_path)
            .arg("--lmlmxtend-json")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn Python ECG worker: {e}"))?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(serialized.as_bytes())
                .map_err(|e| format!("Failed writing payload to Python stdin: {e}"))?;
        }

        let output = child
            .wait_with_output()
            .map_err(|e| format!("Error waiting for Python ECG worker: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Python ECG worker exited with error: {stderr}"));
        }

        let parsed: EcgAnalysisResponse = serde_json::from_slice(&output.stdout)
            .map_err(|e| format!("Invalid JSON response from ECG worker: {e}"))?;

        if parsed.status != "ok" {
            return Err(parsed
                .error
                .unwrap_or_else(|| "Unknown ECG capability error".to_string()));
        }

        serde_json::to_value(parsed)
            .map_err(|e| format!("Failed serializing capability output: {e}"))
    }
}
