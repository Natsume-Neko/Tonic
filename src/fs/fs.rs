use std::fs;
use std::env;
use std::path::PathBuf;

pub struct PathItem {
    pub path_name: String,
    pub full_path: PathBuf,
}
pub struct FileSystem {
    pub current_dir: PathBuf,
    pub path_items: Vec<PathItem>,
    pub on_item_idx: usize,
    pub on_item_dir: PathBuf,
}

pub enum MoveCurseDirection {
    Up,
    Down,
}

impl FileSystem {
    pub fn new() -> Self {
        let current_dir = env::current_dir().unwrap();
        let path_items = Self::get_path_items(&current_dir);
        let on_item_dir = path_items.get(0).unwrap().full_path.clone();
        Self {
            current_dir,
            path_items,
            on_item_idx: 0,
            on_item_dir
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
        let current_path = &self.path_items.get(self.on_item_idx).unwrap().full_path;
        if current_path.is_dir() {
            self.update_current_directory(&current_path_name);
            return true;
        }
        false
    }

    pub fn update_current_directory(&mut self, next_dir: &String) {
        let new_path = self.current_dir.join(next_dir).canonicalize().unwrap();
        self.current_dir = new_path;
        self.path_items = Self::get_path_items(&self.current_dir);
        self.on_item_idx = 0;
        self.update_on_item_dir();
    }

    pub fn update_current_on(&mut self, direction: MoveCurseDirection) -> bool {
        let len = self.path_items.len();
        match direction {
            MoveCurseDirection::Up => {
                if self.on_item_idx > 0 {
                    self.on_item_idx -= 1;
                }
                // self.on_item_idx = (self.on_item_idx + len - 1) % len;
            }
            MoveCurseDirection::Down => {
                if self.on_item_idx + 1 < len {
                    self.on_item_idx += 1;
                }
                // self.on_item_idx = (self.on_item_idx + 1) % len;
            }
        }
        self.update_on_item_dir();
        true
    }

    fn update_on_item_dir(&mut self) {
        self.on_item_dir = self.path_items.get(self.on_item_idx).unwrap().full_path.clone();
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