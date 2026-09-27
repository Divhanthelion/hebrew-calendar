//! The page, built into the binary so the server needs no files beside it.
//! The desktop window loads the same files from `frontend/`.

pub const INDEX: &str = include_str!("../frontend/index.html");
pub const APP_JS: &str = include_str!("../frontend/app.js");
pub const APP_CSS: &str = include_str!("../frontend/app.css");
pub const ICON: &str = include_str!("../frontend/icon.svg");
