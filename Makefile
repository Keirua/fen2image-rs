GO_BIN=go

.PHONY: build clean fmt test

build:
	cargo build

clean:
	cargo clean

fmt:
	cargo fmt

test:
	cargo test
