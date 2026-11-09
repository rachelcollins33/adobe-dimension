//! Windows registry read helpers for Adobe Dimension.

pub const APP_KEY: &str = "Software\\Adobe\\Adobe Dimension";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Adobe Dimension";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Adobe Dimension") },
        RegValue { name: "Version",     value: String::from("v2.0.1") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Adobe Dimension") },
        RegValue { name: "Publisher",       value: String::from("Adobe") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Adobe Dimension\\uninstall.exe\"") },
    ]
}
