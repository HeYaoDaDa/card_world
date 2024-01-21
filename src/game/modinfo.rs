use serde::Deserialize;
use std::{
    fs::{self, File},
    io::Read,
    path::PathBuf,
};

#[derive(Default, Deserialize)]
pub struct Modinfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub authors: Vec<String>,
    pub maintainers: Vec<String>,
    pub version: String,
    pub dependencies: Vec<String>,
    pub obsolete: bool,
    #[serde(skip)]
    pub path: String,
}

pub fn get_all_modinfo_json() -> Vec<Modinfo> {
    let paths = get_all_modinfo_path_by_path("assets/mods");
    let mut modinfos = Vec::new();
    for path in paths {
        let mut modinfo_file =
            File::open(&path).expect(&format!("Open {} fail", path.to_string_lossy()));
        let mut modinfo_json = String::new();
        modinfo_file
            .read_to_string(&mut modinfo_json)
            .expect(&format!("Read {} fail", path.to_string_lossy()));
        let mut modinfo: Modinfo = serde_json::from_str(&modinfo_json).expect(&format!(
            "JSONify {} to Modinfo fail",
            path.to_string_lossy()
        ));
        modinfo.path = path.parent().unwrap_or(&path).to_string_lossy().into();
        modinfos.push(modinfo);
    }
    modinfos
}

fn get_all_modinfo_path_by_path(path: &str) -> Vec<PathBuf> {
    if let Ok(entries) = fs::read_dir(path) {
        let mut dirs = Vec::new();
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() && entry.file_name() == "modinfo.json" {
                        return vec![entry.path()];
                    } else if metadata.is_dir() {
                        dirs.push(entry);
                    }
                }
            }
        }
        let mut result = Vec::new();
        for entry in dirs {
            let paths = get_all_modinfo_path_by_path(entry.path().to_str().unwrap());
            if !paths.is_empty() {
                result.extend(paths);
            }
        }
        return result;
    }
    Vec::new()
}
