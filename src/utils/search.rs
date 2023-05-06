use std::fs;

const MAX_DEPTH: usize = 20;

#[derive(Clone, Debug)]
pub struct Folder {
    pub path: String,
    pub size: Option<usize>,
}

pub fn find_target_folders(start_path: &str, target_folder: &str) -> Vec<Folder> {
    fn traverse(path: &str, target_folder: &str, folders: &mut Vec<Folder>, count: usize) {
        let metadata = fs::metadata(path).unwrap();

        if count > MAX_DEPTH {
            return;
        }

        if metadata.is_file() {
            return;
        }

        // normalizing path because windows *sigh*
        if path.replace('\\', "/").split('/').last().unwrap() == target_folder {
            folders.push(Folder {
                path: path.to_string(),
                size: Some(calculate_folder_size(path)),
            });
            return;
        }

        for dir in fs::read_dir(path).unwrap() {
            let child = dir.unwrap().path();
            let child = child.to_str().unwrap();
            traverse(child, target_folder, folders, count + 1);
        }
    }

    let mut folders = vec![];

    traverse(start_path, target_folder, &mut folders, 0);

    return folders;
}

fn calculate_folder_size(path: &str) -> usize {
    let mut total: usize = 0;

    for dir in fs::read_dir(path).unwrap() {
        let child = dir.unwrap();
        let metadata = child.metadata().unwrap();

        if metadata.is_file() {
            total += metadata.len() as usize;
            continue;
        }

        if metadata.is_symlink() {
            continue;
        }

        total += calculate_folder_size(child.path().to_str().unwrap());
    }

    return total;
}
