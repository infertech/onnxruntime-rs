#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/linux/x86_64/bindings.rs"
));

#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/linux/aarch64/bindings.rs"
));

#[cfg(all(target_os = "linux", target_arch = "arm"))]
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/generated/linux/arm/bindings.rs"
));
