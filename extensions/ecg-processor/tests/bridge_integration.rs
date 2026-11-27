use lmlmxtend_ext_ecg_processor::EcgProcessorExtension;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Helper: Generates a synthetic 12-lead ECG (.mat + .hea) at 500 Hz (60 BPM)
fn create_synthetic_ecg_pair(dir: &Path) -> PathBuf {
    let mat_path = dir.join("synthetic_patient.mat");
    let hea_path = dir.join("synthetic_patient.hea");

    // 1. Write a standard PhysioNet WFDB .hea header file
    let hea_content = "\
synthetic_patient 12 500 5000
synthetic_patient.mat 16+24 1000/mV 16 0 0 0 0 I
synthetic_patient.mat 16+24 1000/mV 16 0 0 0 0 II
synthetic_patient.mat 16+24 1000/mV 16 0 0 0 0 III
synthetic_patient.mat 16+24 1000/mV 16 0 0 0 0 aVR
synthetic_patient.mat 16+24 1000/mV 16 0 0 0 0 aVL
synthetic_patient.mat 16+24 1000/mV 16 0 0 0 0 aVF
synthetic_patient.mat 16+24 1000/mV 16 0 0 0 0 V1
synthetic_patient.mat 16+24 1000/mV 16 0 0 0 0 V2
synthetic_patient.mat 16+24 1000/mV 16 0 0 0 0 V3
synthetic_patient.mat 16+24 1000/mV 16 0 0 0 0 V4
synthetic_patient.mat 16+24 1000/mV 16 0 0 0 0 V5
synthetic_patient.mat 16+24 1000/mV 16 0 0 0 0 V6
# Age: 45
# Sex: Male
# Dx: 426783006
";
    fs::write(&hea_path, hea_content).expect("Failed to write synthetic .hea file");

    // 2. Use scipy.io.savemat to write a 12x5000 float64 signal with 1 Hz R-peaks (60 BPM)
    let py_gen = format!(
        "import numpy as np; from scipy.io import savemat; \
         t = np.linspace(0, 10, 5000); \
         sig = np.zeros((12, 5000)); \
         sig[1] = np.exp(-((t % 1.0 - 0.5) ** 2) / 0.001) * 1.5; \
         savemat(r'{}', {{'val': sig}})",
        mat_path.display()
    );

    let status = Command::new("python3")
        .arg("-c")
        .arg(&py_gen)
        .status()
        .expect("Failed to execute python3 to build test fixture");
    assert!(status.success(), "Python fixture generator failed");

    mat_path
}

#[test]
fn test_ecg_bridge_end_to_end() {
    let temp_dir = std::env::temp_dir().join("lmlmxtend_ecg_test");
    fs::create_dir_all(&temp_dir).unwrap();
    let mat_file = create_synthetic_ecg_pair(&temp_dir);

    // Point directly to the script relative to the workspace root
    let script_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ecg_preprocessing.py");
    let extension = EcgProcessorExtension::new(script_path, "python3");

    let request = json!({
        "file_path": mat_file.to_string_lossy(),
        "apply_filter": true
    });

    let response = extension
        .execute(request)
        .expect("Rust-to-Python ECG bridge execution failed");

    assert_eq!(response["status"], "ok");
    assert_eq!(response["capability"], "signal.ecg.analyze");
    assert_eq!(response["num_leads"], 12);
    assert_eq!(response["samples"], 5000);

    let bpm = response["vitals"]["bpm"].as_f64().unwrap();
    assert!((bpm - 60.0).abs() < 2.0, "Expected ~60 BPM, got {bpm}");

    // Cleanup temporary test files
    let _ = fs::remove_dir_all(&temp_dir);
}

#[test]
fn test_rejects_oversized_payload_over_8mib() {
    let script_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("ecg_preprocessing.py");
    let extension = EcgProcessorExtension::new(script_path, "python3");

    // Generate a 9 MiB dummy string to verify the 8 MiB safety boundary
    let oversized_string = "A".repeat(9 * 1024 * 1024);
    let request = json!({
        "file_path": oversized_string
    });

    let result = extension.execute(request);
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("exceeds LMLMxtend 8 MiB limit"));
}
