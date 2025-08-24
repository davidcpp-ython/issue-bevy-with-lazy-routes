#!/usr/bin/env just --justfile

# Default recipe - show available commands
default:
    @just --list

# Helper function to determine the operating system
os := `uname`

# Install cargo-leptos if not already installed
install-cargo-leptos:
    rustup update
    cargo install cargo-binstall
    cargo binstall cargo-leptos --version 0.2.32

# Packages the project. Assumes the target has been built.
package target:
    rm -rf ./build/
    mkdir -p ./build/
    cp ./target/{{target}}/issue-bevy-with-lazy-routes ./build/issue-bevy-with-lazy-routes
    cp ./target/{{target}}/hash.txt ./build/hash.txt || true
    cp -r ./site ./build/site

# Compiles the project
compile target:
    if [ "{{target}}" = "release" ]; then \
        cargo leptos build --release --split; \
    elif [ "{{target}}" = "debug" ]; then \
        cargo leptos build --split; \
    else \
        echo 'Invalid target'; \
        exit 1; \
    fi

# Builds the project
build target:
    just compile {{target}}
    just package {{target}}

# Run the project (mimics lawren's just run debug)
run target:
    just build {{target}}
    cd build && ./issue-bevy-with-lazy-routes

# Clean build artifacts
clean:
    cargo clean
    rm -rf site/
    rm -rf build/