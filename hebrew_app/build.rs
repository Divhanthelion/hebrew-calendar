//! Build script for Tauri

#[cfg(feature = "gui")]
fn main() {
    tauri_build::build()
}

#[cfg(not(feature = "gui"))]
fn main() {
    // No build steps needed for server-only build
}
