# ARCH:=aarch64-linux-gnu
# CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=$(ARCH)-gcc
# CC_aarch64_unknown_linux_gnu=$(ARCH)-gcc
# CXX_aarch64_unknown_linux_gnu=$(ARCH)-g++
# export

.PHONY: all
all: target/release/librustini.rlib docs test

docs:
	cargo doc --release

target/release/librustini.rlib:
	cargo build --release
	
.PHONY: test
test:
	cargo test --release

.PHONY: clean
clean:
	$(RM) -rf target Cargo.lock doc

# .PHONY: arm64
# arm64:
# 	rustup target add aarch64-unknown-linux-gnu
# 	cargo build --release --target aarch64-unknown-linux-gnu
