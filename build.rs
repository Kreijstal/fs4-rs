fn main() {
    // Detect if we're using nightly compiler
    let version = rustc_version::version_meta().unwrap();
    if version.channel == rustc_version::Channel::Nightly {
        println!("cargo:rustc-cfg=nightly");
    }
}
