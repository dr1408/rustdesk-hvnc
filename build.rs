fn main() {
    // Generate version info
    hbb_common::gen_version();

    #[cfg(windows)]
    {
        let file = "src/platform/windows.cc";

        cc::Build::new()
            .cpp(true)
            .file(file)
            .compile("windows");

        println!("cargo:rustc-link-lib=WtsApi32");
        println!("cargo:rerun-if-changed={}", file);
    }

    // Ensure build.rs changes trigger rebuild
    println!("cargo:rerun-if-changed=build.rs");
}
