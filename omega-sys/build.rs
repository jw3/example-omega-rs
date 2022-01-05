fn main() {
    println!("cargo:rustc-link-search=.local");
    println!("cargo:rustc-link-lib=dylib=omega_edit");
}
