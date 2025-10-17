# OpticalC Makefile
# Available commands:
#   help     - Show this help message
#   test     - Run cargo test
#   wasm-pack - Build WASM package with wasm-pack
#   publish  - Publish both cargo and wasm packages

.PHONY: help test wasm-pack publish

# Default target - show help
help:
	@echo "Available commands:"
	@echo "  help     - Show this help message"
	@echo "  test     - Run cargo test"
	@echo "  wasm-pack - Build WASM package with wasm-pack"
	@echo "  publish  - Publish both cargo and wasm packages"

# Run tests
test:
	cargo test

# Build WASM package
wasm-pack:
	wasm-pack build --release --target bundler --features=wasm

# Publish both packages
publish:
	cargo publish
	wasm-pack publish
