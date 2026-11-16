cargo test --workspace
cargo run --example basic_extension -p lmlmxtend-runtime

# Generate a new creative extension
cargo run -p lmlmxtend-extension-forge -- new --name scene-composer --category creative --capability creative.compose
cargo run -p lmlmxtend-extension-forge -- new --name scene-composer --category creative --capability creative.compose
