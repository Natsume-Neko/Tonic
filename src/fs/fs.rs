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
    on_item_idx: usize,
}

pub enum MoveCurseDirection {
    Up,
    Down,
}

impl FileSystem {
    pub fn new() -> Self {
        let current_dir = env::current_dir().unwrap();
        let path_items = Self::get_path_items(&current_dir);
        Self {
            current_dir,
            path_items,
            on_item_idx: 0
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

    pub fn get_to_curse_on(&mut self) -> bool {
        let current_path_name = self.path_items.get(self.on_item_idx).unwrap().path_name.clone();
        self.update_current_directory(&current_path_name);
        true
    }

    pub fn update_current_directory(&mut self, next_dir: &String) {
        let new_path = self.current_dir.join(next_dir).canonicalize().unwrap();
        self.current_dir = new_path;
        self.path_items = Self::get_path_items(&self.current_dir);
        self.on_item_idx = 0;
    }

    pub fn update_current_on(&mut self, direction: MoveCurseDirection) -> bool {
        let len = self.path_items.len();
        match direction {
            MoveCurseDirection::Up => {
                self.on_item_idx = (self.on_item_idx + len - 1) % len;
                true
            }
            MoveCurseDirection::Down => {
                self.on_item_idx = (self.on_item_idx + 1) % len;
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_current_paths() {
        let mut fs = FileSystem::new();
        println!("{}", fs.current_dir.file_name().unwrap().to_string_lossy().to_string());
        for item in fs.path_items.iter() {
            println!("{}", item.path_name);
        }
        fs.get_to_curse_on();
        println!("{}", fs.current_dir.file_name().unwrap().to_string_lossy().to_string());
        for item in fs.path_items.iter() {
            println!("{}", item.path_name);
        }
        let len = fs.path_items.len();
        for _ in 0..30 {
            fs.update_current_on(MoveCurseDirection::Up);
        }
        for _ in 0..20 {
            fs.update_current_on(MoveCurseDirection::Down);
        }
        assert_eq!(fs.on_item_idx, (len * 10 - 10) % len);
        assert!(fs.path_items.len() >= 1)
    }
}