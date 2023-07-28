use std::fs::File;
use crate::domain::hadith::models::Mousned;
use std::path::Path;
use serde_yaml;
use std::fs::read_dir;

fn build_mousned_from_file(file_path: &Path, sahib: Option<String>) -> Result<Mousned, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut mousned: Mousned = serde_yaml::from_reader(file)?;
    mousned.sahib = sahib;
    Ok(mousned)
}

pub fn load(dir_path: &Path, sahib: Option<String>) -> Result<Vec<Mousned>, Box<dyn std::error::Error>> {
    let mut mousned_vec: Vec<Mousned> = Vec::new();

    if dir_path.is_dir() {
        for entry in read_dir(dir_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let new_sahib = path.file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| name.to_string());
                
                let mut new_mousned = Mousned {
                    ahadith: vec![],
                    sahib: new_sahib.clone(),
                };
                
                for file_entry in read_dir(path)? {
                    let file_path = file_entry?.path();
                    let mousned = build_mousned_from_file(&file_path, new_sahib.clone())?;
                    new_mousned.ahadith.extend(mousned.ahadith);
                }

                mousned_vec.push(new_mousned);
            } else {
                let file_stem = path.file_stem()
                    .and_then(|name| name.to_str())
                    .map(|name| name.to_string());
                let mousned = build_mousned_from_file(&path, file_stem)?;
                mousned_vec.push(mousned);
            }
        }
    }
    Ok(mousned_vec)
}