fn main() {
    tauri_build::build();

    // let mut windows = tauri_build::WindowsAttributes::new();
    // windows = windows.app_manifest(include_str!("./app.manifest"));
    // let attributes = tauri_build::Attributes::new().windows_attributes(windows);
    // tauri_build::try_build(attributes).expect("failed to run build script");
}
