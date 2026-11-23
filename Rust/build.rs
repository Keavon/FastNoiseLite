// Points the "wgsl" feature flag's `WGSL_SOURCE` at the WGSL port: the repository's own when built from it,
// otherwise the copy that the publish workflow places beside `Cargo.toml`, since a published crate only carries its own folder
fn main() {
    if std::env::var_os("CARGO_FEATURE_WGSL").is_none() {
        return;
    }

    let manifest_directory = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let port = [manifest_directory.join("../WGSL/FastNoiseLite.wgsl"), manifest_directory.join("FastNoiseLite.wgsl")]
        .into_iter()
        .find(|path| path.exists())
        .expect("the WGSL port should be in the repository's WGSL folder, or copied beside Cargo.toml for publishing");

    println!("cargo:rerun-if-changed={}", port.display());
    println!("cargo:rustc-env=FASTNOISE_LITE_WGSL_PORT={}", port.display());
}
