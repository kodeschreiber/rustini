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
