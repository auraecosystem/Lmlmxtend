if __name__ == "__main__":
    import sys

    # Pass the ECG dataset folder path as an argument, or default to "./data"
    dataset_path = sys.argv[1] if len(sys.argv) > 1 else "./data"

    if not os.path.exists(dataset_path):
        print(f"Dataset directory '{dataset_path}' not found. Pass a valid path: python3 ecg_preprocessing.py /path/to/ecg_data")
        sys.exit(1)

    gender, age, labels, ecg_filenames = import_key_data(dataset_path)
    age_clean, gender_binary = import_gender_and_age(age, gender)
    print(f"Loaded {len(ecg_filenames)} ECG records successfully.")
