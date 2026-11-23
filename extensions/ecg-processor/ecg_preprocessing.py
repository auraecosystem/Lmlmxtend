import os
import sys
import json
import math
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns
from scipy.io import loadmat
from scipy.signal import butter, filtfilt, find_peaks, peak_widths, savgol_filter
from sklearn.preprocessing import MultiLabelBinarizer, LabelEncoder
from sklearn.model_selection import StratifiedKFold
from keras.preprocessing.sequence import pad_sequences

# Enforce Lmlmxtend 8 MiB payload boundary (8,388,608 bytes)
MAX_LMLM_PAYLOAD_BYTES = 8 * 1024 * 1024


# ==============================================================================
# 1. DATA INGESTION & DEMOGRAPHIC CLEANING
# ==============================================================================
def load_challenge_data(filename):
    x = loadmat(filename)
    data = np.asarray(x['val'], dtype=np.float64)
    new_file = filename.replace('.mat', '.hea')
    with open(new_file, 'r') as f:
        header_data = f.readlines()
    return data, header_data


def clean_up_gender_data(gender):
    gender = np.asarray(gender)
    gender[np.where(np.isin(gender, ["Male", "male", "M"]))] = 0
    gender[np.where(np.isin(gender, ["Female", "female", "F"]))] = 1
    gender[np.where(gender == "NaN")] = 2
    return gender.astype(int)


def clean_up_age_data(age):
    age = np.asarray(age)
    age[np.where(age == "NaN")] = -1
    return age.astype(int)


def import_gender_and_age(age, gender):
    gender_binary = clean_up_gender_data(gender)
    age_clean = clean_up_age_data(age)
    return age_clean, gender_binary


def import_key_data(path):
    gender, age, labels, ecg_filenames = [], [], [], []
    for subdir, _, files in sorted(os.walk(path)):
        for filename in files:
            filepath = os.path.join(subdir, filename)
            if filepath.endswith(".mat"):
                _, header_data = load_challenge_data(filepath)
                labels.append(header_data[15][5:-1])
                ecg_filenames.append(filepath)
                gender.append(header_data[14][6:-1])
                age.append(header_data[13][6:-1])
    return gender, age, labels, ecg_filenames


# ==============================================================================
# 2. SIGNAL PROCESSING EXTENSIONS (Filtering, R-Peaks, & ONNX Formatting)
# ==============================================================================
def bandpass_filter(signal, lowcut=0.5, highcut=45.0, fs=500.0, order=4):
    """Removes baseline wander and high-frequency noise from a 12-lead ECG."""
    nyquist = 0.5 * fs
    low = lowcut / nyquist
    high = highcut / nyquist
    b, a = butter(order, [low, high], btype='band')
    return filtfilt(b, a, signal, axis=-1)


def extract_r_peaks_and_bpm(lead_signal, fs=500.0):
    """Detects R-peaks on a single lead (e.g., Lead II) and computes BPM."""
    smoothed = savgol_filter(lead_signal, window_length=15, polyorder=3)
    min_distance = int(0.3 * fs)  # Minimum 300ms between heartbeats (~200 BPM max)
    peaks, _ = find_peaks(smoothed, distance=min_distance, prominence=0.3)

    if len(peaks) < 2:
        return {"r_peaks": peaks.tolist(), "bpm": 0.0, "rr_std_ms": 0.0}

    rr_intervals_sec = np.diff(peaks) / fs
    bpm = float(60.0 / np.mean(rr_intervals_sec))
    rr_std_ms = float(np.std(rr_intervals_sec) * 1000.0)
    return {
        "r_peaks": peaks.tolist(),
        "bpm": round(bpm, 2),
        "rr_std_ms": round(rr_std_ms, 2)
    }


def export_batch_for_onnx(ecg_filenames, maxlen=5000, apply_filter=True):
    """Prepares float32 (N, 12, 5000) tensors for LMLM ONNX inference."""
    batch = []
    for path in ecg_filenames:
        data, _ = load_challenge_data(path)
        if apply_filter:
            data = bandpass_filter(data)
        data = pad_sequences(data, maxlen=maxlen, truncating='post', padding='post', dtype='float32')
        batch.append(data)
    return np.asarray(batch, dtype=np.float32)


# ==============================================================================
# 3. LABEL ENCODING, CROSS-VALIDATION & BATCHING
# ==============================================================================
def make_undefined_class(labels, df_unscored):
    df_labels = pd.DataFrame(labels)
    for i in range(len(df_unscored.iloc[:, 1])):
        df_labels.replace(
            to_replace=str(df_unscored.iloc[i, 1]),
            inplace=True,
            value="undefined class",
            regex=True
        )
    return df_labels


def onehot_encode(df_labels):
    one_hot = MultiLabelBinarizer()
    y = one_hot.fit_transform(df_labels[0].str.split(pat=','))
    y = np.delete(y, -1, axis=1)
    return y, one_hot.classes_[0:-1]


def get_labels_for_all_combinations(y):
    return LabelEncoder().fit_transform([''.join(str(l)) for l in y])


def split_data(labels, y_all_combo):
    return list(StratifiedKFold(n_splits=10, shuffle=True, random_state=42).split(labels, y_all_combo))


def generate_validation_data_with_demo_data(ecg_filenames, y, gender, age, test_order_array):
    y_val = y[test_order_array]
    selected_files = ecg_filenames[test_order_array]

    timeseries = []
    for name in selected_files:
        data, _ = load_challenge_data(name)
        data = pad_sequences(data, maxlen=5000, truncating='post', padding='post')
        timeseries.append(data)

    X_val = np.asarray(timeseries).reshape(selected_files.shape[0], 5000, 12)
    demographics_val = np.column_stack((age[test_order_array], gender[test_order_array]))
    return X_val, demographics_val, y_val


# ==============================================================================
# 4. LMLMxtend CAPABILITY DISPATCHER & STANDALONE CLI ENTRY POINT
# ==============================================================================
def run_lmlmxtend_capability(raw_payload: str):
    """Handles capability calls from lmlmxtend-runtime (signal.ecg.analyze)."""
    if len(raw_payload.encode("utf-8")) > MAX_LMLM_PAYLOAD_BYTES:
        return {"status": "error", "error": "Payload exceeds LMLMxtend 8 MiB limit"}

    request = json.loads(raw_payload)
    capability = request.get("capability", "signal.ecg.analyze")
    file_path = request.get("file_path")

    if not file_path or not os.path.exists(file_path):
        return {"status": "error", "error": f"ECG file not found: {file_path}"}

    if capability != "signal.ecg.analyze":
        return {"status": "error", "error": f"Unsupported capability: {capability}"}

    apply_filter = bool(request.get("apply_filter", True))
    data, _header = load_challenge_data(file_path)

    if data.ndim != 2 or data.shape[0] != 12:
        return {
            "status": "error",
            "error": f"Expected a 12-lead ECG matrix, got shape {data.shape}"
        }

    processed = bandpass_filter(data) if apply_filter else data
    # Lead II (index 1) is used for rhythm analysis.
    vitals = extract_r_peaks_and_bpm(processed[1])

    return {
        "status": "ok",
        "capability": capability,
        "contract_version": "v1",
        "num_leads": int(data.shape[0]),
        "samples": int(data.shape[1]),
        "vitals": vitals
    }


if __name__ == "__main__":
    # Mode 1: Invoked by Lmlmxtend runtime via --lmlmxtend-json
    if len(sys.argv) > 1 and sys.argv[1] == "--lmlmxtend-json":
        payload = sys.stdin.read()
        result = run_lmlmxtend_capability(payload)
        print(json.dumps(result))
        sys.exit(0)

    # Mode 2: Standalone CLI dataset preprocessing
    dataset_path = sys.argv[1] if len(sys.argv) > 1 else "./Lmlm-Data"
    if not os.path.exists(dataset_path):
        print(f"Usage: python3 ecg_preprocessing.py <dataset_folder> OR --lmlmxtend-json")
        sys.exit(1)

    gender, age, labels, ecg_filenames = import_key_data(dataset_path)
    age_clean, gender_binary = import_gender_and_age(age, gender)
    print(f"Processed {len(ecg_filenames)} ECG records | Age shape: {age_clean.shape} | Gender shape: {gender_binary.shape}")
