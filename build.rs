fn main() {
    // todo: sort by cfg and download models
    println!("cargo:rerun-if-changed=build.rs");
    if cfg!(feature = "openvtuber") {}
    if cfg!(feature = "dlib") {}
}
