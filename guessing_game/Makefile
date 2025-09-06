.DEFAULT_GOAL := build

BUILD_NAME=guessing_game

.PHONY: fmt clean build
fmt:
	rustfmt src/main.rs

clean: fmt
	rm -rf target/debug/$(BUILD_NAME)

build: clean
	cargo build
