fn main() {
    println!("cargo:rerun-if-changed=cpp");
    cxx_build::bridge("src/lib.rs")
        .include("cpp/include")
        .flag_if_supported("-std=c++20")
        .opt_level(2)
        .compile("frc_wpi");
    println!("cargo:rerun-if-changed=src/lib.rs");
    cxx_build::bridge("src/rev/ffi.rs")
        .include("cpp/include")
        .flag_if_supported("-std=c++20")
        .opt_level(2)
        .compile("frc_rev");
    println!("cargo:rerun-if-changed=src/rev/ffi.rs");
}
