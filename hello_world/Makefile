.DEFAULT_GOAL := build

BUILD_NAME=hello_world

.PHONY: fmt clean build
fmt:
	rustfmt main.rs
clean: fmt
	rm -rf $(BUILD_NAME) 
build: clean
	rustc main.rs -o $(BUILD_NAME)
