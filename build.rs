use std::{
    env::{
        consts::{ARCH, DLL_PREFIX, DLL_SUFFIX, OS},
        var,
    },
    fs,
    path::Path,
};

fn library_filename<S: AsRef<str>>(name: S) -> String {
    let name = name.as_ref();
    let mut string = String::with_capacity(name.len() + DLL_PREFIX.len() + DLL_SUFFIX.len());
    string.push_str(DLL_PREFIX);
    string.push_str(name);
    string.push_str(DLL_SUFFIX);
    string
}

fn main() {
    let target_os = var("TARGET").unwrap();
    let profile = var("PROFILE").unwrap();

    let dsts = [
        Path::new("target")
            .join(target_os)
            .join(&profile)
            .join("deps"),
        Path::new("target").join(profile).join("deps"),
    ];

    let filename = library_filename("pdfium");
    let src_path = Path::new("include").join(OS).join(ARCH).join(&filename);

    for dst in dsts.iter() {
        fs::create_dir_all(dst).expect("unable to create necessary directories for the {}");
        fs::copy(&src_path, dst.join(&filename)).unwrap_or_else(|e| {
            panic!("Unable to copy the dylib to {dst:?}'s deps folder. Err: {e}",)
        });
    }

    println!("cargo:rustc-link-lib=dylib=pdfium");
    println!(
        "cargo:rustc-link-search=native={}",
        Path::new("include").join(OS).join(ARCH).to_str().unwrap()
    );
}
