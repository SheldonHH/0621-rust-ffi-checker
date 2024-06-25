#!/bin/bash

# Build Rust library
cargo build --release

# Compile C++ code and link with Rust library
# g++ -fsanitize=address -o main main.cpp -Ltarget/release -lrust_lib -lpthread -Wl,-rpath,target/release
# ASAN_OPTIONS=detect_stack_use_after_return=1 ./main



