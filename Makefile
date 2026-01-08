BINARY_NAME = fanmon
INSTALL_PATH = $(HOME)/.local/bin

.PHONY: build install clean deps

deps:
	@which cargo > /dev/null || (echo "Error: Rust/Cargo not found. Install from https://rustup.rs/" && exit 1)

build: deps
	cargo build --release

install: build
	mkdir -p $(INSTALL_PATH)
	cp target/release/$(BINARY_NAME) $(INSTALL_PATH)/

clean:
	cargo clean
