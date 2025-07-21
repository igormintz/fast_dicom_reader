use walkdir::WalkDir;
use std::path::PathBuf;

pub fn get_dicom_paths_from_folder(
    folder_path: &str,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    Ok(WalkDir::new(folder_path)
        .min_depth(1) // Skip the root directory itself
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().to_path_buf())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map_or(false, |name| name != ".DS_Store")
        })
        .collect())
}

