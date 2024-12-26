use std::{fs::read_dir, path::{Path, PathBuf}};

use crate::db_manager::{get_connection, get_documents_from_category, Category};


pub fn scan_for_new_files(category : &Category) ->Result<Vec<&Path>, String> {
    let connection = get_connection(db_path)
    let documents_from_category = get_documents_from_category(connection, category_id)
}

fn get_files_with_extensions(path: &Path, extensions: Vec<&str>) -> Result<Vec<PathBuf>, String> {
    Ok(get_all_files(path)?
        .into_iter()
        .filter(|p| p.extension().map_or(false, |e| e.to_str().map_or(false, |e| extensions.contains(&e))))
        .collect::<Vec<PathBuf>>())
}

fn get_all_files(path : &Path) -> Result<Vec<PathBuf>, String> {
    if path.is_file() {
        return Err("Could not scan path because it is not a directory".to_string());
    }
    else {
        let mut files: Vec<PathBuf> = Vec::new();
        let entries = read_dir(path)
            .map_err(|e| format!("An error occured while scanning path {} : {}", path.display(), e))?;
        for entry in entries {
            let entry_path = entry
                .map_err(|e| format!("An error occured while scanning path : {}", e))?.path();
            if entry_path.is_dir() {
                files.append(&mut get_all_files(&entry_path)?);
            }
            else {
                files.push(entry_path);
            }
        }
        return Ok(files);
    }
}