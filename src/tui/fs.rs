use std::fs;
use std::env;
use std::path::PathBuf;

struct PathItem {
    path_name: String,
    full_path: PathBuf,
}
pub struct FileSystem {
    current_dir: PathBuf,
    path_items: Vec<PathItem>,
}

impl FileSystem {
    pub fn new() -> Self {
        let current_dir = env::current_dir().unwrap();
        let mut path_items = Self::get_path_items(&current_dir);
        Self {
            current_dir,
            path_items
        }
    }

    fn get_path_items(current_dir: &PathBuf) -> Vec<PathItem> {
        let entries = fs::read_dir(&current_dir).unwrap();
        let mut path_items = vec![
            PathItem {
                path_name: "..".into(),
                full_path: current_dir.join("..").canonicalize().unwrap()
            },
        ];
        for entry in entries {
            let entry = entry.unwrap();
            let path_name = entry.file_name().to_string_lossy().to_string();
            let path = entry.path();
            path_items.push(
                PathItem {
                    path_name,
                    full_path: path,
                }
            );
        };
        path_items
    }

    pub fn update_current_directory(&mut self, new_path: &String) {
        self.current_dir = PathBuf::from(new_path);
        self.path_items = Self::get_path_items(&self.current_dir);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_current_paths() {
        let fs = FileSystem::new();
        println!("{}", fs.current_dir.file_name().unwrap().to_string_lossy().to_string());
        for item in fs.path_items.iter() {
            println!("{}", item.path_name);
        }
        assert!(fs.path_items.len() >= 1)
    }
}