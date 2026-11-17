.PHONY: verify smoke test fmt lint examples forge-help

verify:
	bash scripts/verify.sh

smoke:
	bash scripts/smoke.sh

test:
	cargo test --workspace

fmt:
	cargo fmt --all -- --check

lint:
	cargo clippy --workspace --all-targets -- -D warnings

examples:
	cargo run --example basic_extension -p lmlmxtend-runtime
	cargo run --example basic_agent -p lmlmxtend-agent-runtime

forge-help:
	cargo run -p lmlmxtend-extension-forge -- help
