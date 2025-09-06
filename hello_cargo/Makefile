.DEFAULT_GOAL := copy_exec

BUILD_NAME=hello_cargo

.PHONY = fmt clean build copy_exec
fmt:
	rustfmt src/main.rs
clean: fmt
	rm -rf target/debug/$(BUILD_NAME)
	rm -rf $(BUILD_NAME)
build: clean
	cargo build
copy_exec: build
	cp target/debug/$(BUILD_NAME) .	
