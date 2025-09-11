fn main() {

    if std::env::var_os("CARGO_PRIMARY_PACKAGE").is_none() {
        return;
    }

    let mut resource = winres::WindowsResource::new();
    resource.set_manifest(include_str!("./app.manifest"));

    if let Err(error) = resource.compile() {
        eprint!("{error}");
        std::process::exit(1);
    }

    println!("cargo:rerun-if-changed=build.rs");
}
