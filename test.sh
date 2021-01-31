#! /bin/sh

echo "RUN RUST TESTS"
cargo test

echo "PACKAGE GUIX-EXAMPLE AS A C DYNAMIC LIBRARY"
cargo package --manifest-path ./protocols/guix-example/Cargo.toml  --allow-dirty --no-verify

echo "INSTALL THE ABOVE LIBRARY IN A NEW GUIX ENVIRONMENT"
# Create a binary from the env to test the library
guix environment --container gcc-toolchain --ad-hoc -l ./protocols/guix-example/guix-example.scm  -- gcc test/interop.cpp  ./target/release/libguix_example.so

echo "TEST THE LIBRARY"
./a.out

echo "REMOVE THE BINARY"
rm a.out
