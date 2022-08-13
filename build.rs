fn main() {
    const C_SRC: &str = "src/ffi_constants.c";

    println!("cargo:rerun-if-changed={}", C_SRC);

    cc::Build::new()
        .file(C_SRC)
        .compile("ffi_constants");
}
