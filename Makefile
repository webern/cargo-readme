.PHONY: check build test fmt fmt-check clippy clippy-fix install ci publish-dry-run

check:
	cargo check

build:
	cargo build

test:
	cargo test --all-features

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

clippy:
	cargo clippy --all-features --tests -- -D warnings

install:
	$(eval TMPDIR := $(shell mktemp -d))
	cargo install --path . --root $(TMPDIR)
	rm -rf $(TMPDIR)

ci: check fmt-check clippy test build install

publish-dry-run: ci
	cargo publish --dry-run
