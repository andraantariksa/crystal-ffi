build: build-rust build-crystal

build-rust:
	rustc rust.rs -O --crate-type cdylib -o build/librust.so

build-crystal:
	crystal build crystal.cr --release -o build/crystal_ffi