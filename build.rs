fn main() {
    // Compile the small C library used by the FFI example.
    cc::Build::new()
        .file("examples/ffi_c/adder.c")
        .include("examples/ffi_c")
        .compile("ffi_c");

    println!("cargo:rerun-if-changed=examples/ffi_c/adder.c");
}
