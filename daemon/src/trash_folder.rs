use std::path::PathBuf;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TrashInfoFile {
    uuid: Uuid,
    original_path: PathBuf,
    deleted_at: Duration,
}

pub fn add_files_to_trash(
    files: &[PathBuf],
    trash_folder_path: PathBuf,
) -> Result<(), std::io::Error> {
    if !trash_folder_path.is_dir() {
        println!(
            "Trash folder directory ({:?}) does not exist, creating...",
            trash_folder_path
        );
        std::fs::create_dir(trash_folder_path)?;
    }
    //TODO: Generate UUID, move file with UUID as filename, create UUID.trashinfo file as per TrashInfoFile
    Ok(())
}
