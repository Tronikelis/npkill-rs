use std::fs;

const MAX_DEPTH: usize = 20;

#[derive(Clone, Debug)]
pub struct Folder {
    pub path: String,
    pub size: Option<usize>,
}

pub fn find_target_folders(start_path: &str, target_folder: &str) -> Vec<Folder> {
    fn traverse(path: &str, target_folder: &str, folders: &mut Vec<Folder>, count: usize) {
        if count > MAX_DEPTH {
            return;
        }

        let metadata = fs::metadata(path).unwrap();

        if metadata.is_file() {
            return;
        }

        // normalizing path because windows oi
        if path.replace("\\", "/").split("/").last().unwrap() == target_folder {
            folders.push(Folder {
                path: path.to_string(),
                size: None,
            });
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
