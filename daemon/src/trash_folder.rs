use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TrashInfoFile {
    uuid: Uuid,
    original_path: PathBuf,
    deleted_at: i64,
}

pub fn add_files_to_trash(
    paths: &[PathBuf],
    trash_folder_path: PathBuf,
) -> Result<(), std::io::Error> {
    if !trash_folder_path.is_dir() {
        println!(
            "Trash folder directory ({:?}) does not exist, creating...",
            trash_folder_path
        );
        fs::create_dir(&trash_folder_path)?;
    }

    for original_path in paths {
        let uuid = Uuid::new_v4();
        let deleted_at = SystemTime::now().duration_since(UNIX_EPOCH);
        if deleted_at.is_err() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Can't get current system time!",
            ));
        }
        let mut new_filename = trash_folder_path.join(uuid.to_string());
        move_file(original_path, &new_filename)?;

        let mut trashinfo_file_path = new_filename.clone();
        trashinfo_file_path.pop();
        let mut uuid_with_extension = uuid.to_string();
        uuid_with_extension.push_str(".trashinfo");

        trashinfo_file_path.push(uuid_with_extension);
        let mut trashinfo_file = File::create(&trashinfo_file_path)?;
        let trashinfo = TrashInfoFile {
            uuid,
            original_path: original_path.clone(),
            deleted_at: deleted_at.unwrap().as_secs() as i64,
        };

        trashinfo_file.write_all(
            toml::to_string_pretty(&trashinfo)
                .expect("config should be serializable")
                .as_bytes(),
        )?;
    }
    Ok(())
}

fn same_filesystem(path: &Path, path2: &Path) -> bool {
    let metadata1 = fs::metadata(path);
    let metadata2 = fs::metadata(path2.parent().expect("trash folder should have parent"));

    match (metadata1, metadata2) {
        (Ok(metadata1), Ok(metadata2)) => {
            // Compare device IDs to determine if the paths are on the same one
            metadata1.dev() == metadata2.dev()
        }
        _ => false,
    }
}

fn move_file(path: &Path, dest: &Path) -> Result<(), std::io::Error> {
    // Renaming only works if the src and dest are on the same filesystem, otherwise a full copy is needed
    if same_filesystem(path, dest) {
        fs::rename(path, dest)?;
    } else {
        todo!("make sure this works");
        dbg!(fs::copy(path, dest))?;
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}
