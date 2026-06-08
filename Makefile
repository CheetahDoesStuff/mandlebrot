APP_NAME := mandlebrot

LINUX_TARGET   := x86_64-unknown-linux-gnu
WINDOWS_TARGET := x86_64-pc-windows-gnu
MACOS_INTEL    := x86_64-apple-darwin
MACOS_ARM      := aarch64-apple-darwin

RELEASE_DIR := target
OUT_DIR     := dist

export CARGO_HOME  := /mnt/nvme0/rust/cargo
export RUSTUP_HOME := /home/cheetah/.rustup

.PHONY: all linux windows macos clean help

all: linux windows macos

linux: 
	@echo "Building for linux..."
	@mkdir -p $(OUT_DIR)
	cargo build --target $(LINUX_TARGET) --release
	cp $(RELEASE_DIR)/$(LINUX_TARGET)/release/$(APP_NAME) $(OUT_DIR)/$(APP_NAME)-linux

windows:
	@echo "Building for Windows..."
	@mkdir -p $(OUT_DIR)
	cross build --target $(WINDOWS_TARGET) --release
	cp $(RELEASE_DIR)/$(WINDOWS_TARGET)/release/$(APP_NAME).exe $(OUT_DIR)/$(APP_NAME)-windows.exe

macos: macos-intel macos-arm
macos-intel:
	@echo "Building for macOS (Intel)..."
	@mkdir -p $(OUT_DIR)
	cross build --target $(MACOS_INTEL) --release
	cp $(RELEASE_DIR)/$(MACOS_INTEL)/release/$(APP_NAME) $(OUT_DIR)/$(APP_NAME)-macos-intel
macos-arm:
	@echo "Building for macOS (Apple Silicon)..."
	@mkdir -p $(OUT_DIR)
	cross build --target $(MACOS_ARM) --release
	cp $(RELEASE_DIR)/$(MACOS_ARM)/release/$(APP_NAME) $(OUT_DIR)/$(APP_NAME)-macos-arm

clean:
	cargo clean
	rm -rf $(OUT_DIR)

help:
	@echo "Usage:"
	@echo "  make all          - Build for all platforms"
	@echo "  make linux        - Build for Linux"
	@echo "  make windows      - Build for Windows"
	@echo "  make macos        - Build for macOS (Intel + ARM)"
	@echo "  make macos-intel  - Build for macOS Intel only"
	@echo "  make macos-arm    - Build for macOS Apple Silicon only"
	@echo "  make clean        - Remove build artifacts and dist/"