bash scripts/new-extension.sh --name ecg-processor --category signal --capability signal.ecg.analyze
cargo test -p lmlmxtend-ext-ecg-processor -- --nocapture
