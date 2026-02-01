use std::{fs, path::PathBuf, process::Command};
use freedesktop_desktop_entry::{DesktopEntry, get_languages_from_env};
use plist::Value;

use crate::window::is_mac;


#[derive(Debug, Clone)]
pub struct AppItem {
    pub exec: String,
    pub needs_terminal: bool,
    pub icon: String,
    pub name: String,
    pub description: String,
    pub _path: PathBuf,
}

impl AppItem {
    pub fn launch(&self) {
        if self.needs_terminal {
            dbg!("terminal application");
        } else {
            let _ = Command::new("sh").arg("-c").arg(&self.exec).spawn();
        }
    }
}

pub fn collect_apps() -> Vec<AppItem> {
    if is_mac() {
        return collect_apps_mac()
    }

    let locales = get_languages_from_env();
    freedesktop_desktop_entry::Iter::new(freedesktop_desktop_entry::default_paths())
        .into_iter()
        .filter_map(|p| {
            if let Ok(entry) = DesktopEntry::from_path(p.clone(), Some(&locales)) {
                if entry.no_display() {
                    return None;
                } else {
                    return Some(AppItem {
                        exec: entry.exec().unwrap_or_default().to_string(),
                        needs_terminal: entry.terminal(),
                        icon: entry
                            .icon()
                            .unwrap_or("application-x-executable")
                            .to_string(),
                        name: entry.name(&locales).unwrap_or_default().to_string(),
                        description: entry.comment(&locales).unwrap_or_default().to_string(),
                        _path: p,
                    });
                }
            }

            None
        })
        .collect()
}

fn collect_apps_mac() -> Vec<AppItem> {
    let mut apps = Vec::new();
    let user_path = format!("{}/Applications", std::env::var("HOME").unwrap());
    let paths = vec!["/Applications", &user_path];

    for base_path in paths {
        if let Ok(entries) = fs::read_dir(base_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("app") {
                    let plist_path = path.join("Contents/Info.plist");
                    if let Ok(plist_file) = fs::File::open(&plist_path) &&
                        let Ok(plist) = Value::from_reader(plist_file) {
                            let dict = plist.as_dictionary().unwrap();
                            let name = dict.get("CFBundleName")
                                .or_else(|| dict.get("CFBundleDisplayName"))
                                .and_then(|v| v.as_string())
                                .unwrap_or("")
                                .to_string();
                            let exec = dict.get("CFBundleExecutable")
                                .and_then(|v| v.as_string())
                                .unwrap_or("")
                                .to_string();
                            let icon = dict.get("CFBundleIconFile")
                                .and_then(|v| v.as_string())
                                .unwrap_or("default_icon")
                                .to_string();
                            let description = dict.get("CFBundleGetInfoString")
                                .and_then(|v| v.as_string())
                                .unwrap_or("")
                                .to_string();

                            apps.push(AppItem {
                                exec,
                                needs_terminal: false,
                                icon,
                                name,
                                description,
                                _path: path.clone(),
                            });
                        }
                    
                }
            }
        }
    }

    apps
}

