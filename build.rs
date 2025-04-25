fn main() {
    let target = std::env::var("TARGET").expect("TARGET was not set");
    println!("cargo:warning=Building for target: {}", target);

    println!("cargo:rustc-link-lib=dylib=pdfium");
    println!("cargo:rustc-link-search=native=./include/macos");
}
