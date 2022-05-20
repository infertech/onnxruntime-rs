#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/linux/x86_64/bindings.rs"
));
