BINARY_NAME=linkchecker
CARGO=cargo

.PHONY: all build run test fmt clippy clean

all: build

build:
	$(CARGO) build --release

run:
	$(CARGO) run -- $(ARGS)

test:
	$(CARGO) test

fmt:
	$(CARGO) fmt --all -- --check

clippy:
	$(CARGO) clippy -- -D warnings


clean:
	$(CARGO) clean
