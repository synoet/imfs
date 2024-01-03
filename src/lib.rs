mod error;
mod htm;
use error::CacheError;
use htm::{HashedTreeMap, TreeNode};
use std::fs::{read, read_dir};
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub size: u64,
    pub buffer: Vec<u8>,
    pub location: String,
}

#[derive(Debug, Clone)]
pub struct Directory {
    pub location: String,
    pub created: SystemTime,
    pub modified: SystemTime,
}

impl Directory {
    pub fn new(location: String) -> Self {
        Self {
            location,
            created: SystemTime::now(),
            modified: SystemTime::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FileSystemItem {
    File(File),
    Directory(Directory),
}

trait InitializeFileTree {
    fn initialize_dir(&mut self, location: &str);
}

impl InitializeFileTree for HashedTreeMap<FileSystemItem> {
    fn initialize_dir(&mut self, location: &str) {
        let node_ref = self.get(location).unwrap();

        let dir = match &node_ref.borrow().value {
            FileSystemItem::Directory(dir) => dir.clone(),
            _ => panic!("Not a directory"),
        };

        let dir_path = Path::new(&dir.location);

        let dir_entries = read_dir(dir_path).unwrap();

        for entry in dir_entries {
            let entry = entry.unwrap();
            let metadata = entry.metadata().unwrap();
            let path = entry.path();

            if metadata.is_dir() {
                let dir = Directory::new(path.to_str().unwrap().to_string());
                let item = FileSystemItem::Directory(dir);

                self.insert(
                    node_ref.clone(),
                    path.to_str().unwrap().to_string(),
                    path.file_name().unwrap().to_str().unwrap().to_string(),
                    item,
                );
                self.initialize_dir(path.to_str().unwrap());
            } else {
                let file = File {
                    name: path.file_name().unwrap().to_str().unwrap().to_string(),
                    created: metadata.created().unwrap(),
                    modified: metadata.modified().unwrap(),
                    size: metadata.len(),
                    buffer: read(path.clone()).unwrap(),
                    location: path.to_str().unwrap().to_string(),
                };

                let item = FileSystemItem::File(file);

                self.insert(
                    node_ref.clone(),
                    path.to_str().unwrap().to_string(),
                    path.file_name().unwrap().to_str().unwrap().to_string(),
                    item,
                )
            }
        }
    }
}

trait FileItemDescriptors {
    fn name(&self) -> String;
    fn file_type(&self) -> String;
}

impl FileItemDescriptors for FileSystemItem {
    fn name(&self) -> String {
        match self {
            FileSystemItem::File(file) => file.name.clone(),
            FileSystemItem::Directory(dir) => dir.location.clone(),
        }
    }

    fn file_type(&self) -> String {
        match self {
            FileSystemItem::File(_) => "file".to_string(),
            FileSystemItem::Directory(_) => "directory".to_string(),
        }
    }
}

impl std::fmt::Debug for TreeNode<FileSystemItem> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, type: {}",
            self.value.name(),
            self.value.file_type()
        )
    }
}

pub struct Cache {
    host_location: String,
    tree: HashedTreeMap<FileSystemItem>,
}

impl Cache {
    pub fn new(location: &str) -> Result<Self, CacheError> {
        let path = Path::new(location);

        if !path.exists() {
            return Err(CacheError::LocationDoesNotExistError {
                location: location.to_string(),
            });
        }

        let root_dir = Directory::new(location.to_string());
        let root_node = TreeNode::new(FileSystemItem::Directory(root_dir));
        let mut tree = HashedTreeMap::new(location.to_string(), root_node);

        tree.initialize_dir(location);

        let cache = Self {
            host_location: location.to_string(),
            tree,
        };

        Ok(cache)
    }

    pub fn exists(&self, location: &str) -> bool {
        self.tree.get(location).is_some()
    }

    pub fn read(&self, location: &str) -> Result<FileSystemItem, CacheError> {
        if !self.exists(location) {
            return Err(CacheError::LocationDoesNotExistError {
                location: location.to_string(),
            });
        }
        let node_ref = self.tree.get(location).unwrap();
        let node = node_ref.borrow();
        Ok(node.value.clone())
    }

    pub fn mkdir(&mut self, location: &str) -> Result<(), CacheError> {
        if self.exists(location) {
            return Err(CacheError::LocationAlreadyExistsError {
                location: location.to_string(),
            });
        }

        let location_path = std::path::Path::new(location);
        let parent_path = location_path.parent().unwrap();

        let parent_node_ref = self.tree.get(parent_path.to_str().unwrap());

        match parent_node_ref {
            Some(parent_node_ref) => {
                self.tree.insert(
                    parent_node_ref.clone(),
                    location.to_string(),
                    location_path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                    FileSystemItem::Directory(Directory::new(location.to_string())),
                );
                return Ok(());
            }
            None => {
                return Err(CacheError::LocationDoesNotExistError {
                    location: location.to_string(),
                });
            }
        };
    }

    pub fn write(&mut self, location: &str, name: &str, buffer: Vec<u8>) -> Result<(), CacheError> {
        if self.exists(location) {
            return Err(CacheError::LocationAlreadyExistsError {
                location: location.to_string(),
            });
        }

        let location_path = std::path::Path::new(location);
        let parent_path = location_path.parent().unwrap();

        let parent_node_ref = self.tree.get(parent_path.to_str().unwrap());

        match parent_node_ref {
            Some(parent_node_ref) => {
                self.tree.insert(
                    parent_node_ref.clone(),
                    location.to_string(),
                    location_path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                    FileSystemItem::File(File {
                        name: name.to_string(),
                        created: SystemTime::now(),
                        modified: SystemTime::now(),
                        size: buffer.len() as u64,
                        buffer,
                        location: location.to_string(),
                    }),
                );
                return Ok(());
            }
            None => {
                return Err(CacheError::LocationDoesNotExistError {
                    location: location.to_string(),
                });
            }
        };
    }

    pub fn location(&self) -> &str {
        &self.host_location
    }

    pub fn rm(&mut self, location: &str) -> Result<(), CacheError> {
        if !self.exists(location) {
            return Err(CacheError::LocationDoesNotExistError {
                location: location.to_string(),
            });
        }

        self.tree.remove(location);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn create_cache_with_non_existent_path() {
        use super::*;
        let cache = Cache::new("non-existent-path");
        assert!(cache.is_err());
    }

    #[test]
    fn create_cache_with_existent_path() {
        use super::*;
        let cache = Cache::new("/Users/synoet/dev/imfs");
        assert!(cache.is_ok());
    }

    #[test]
    fn read() {
        use super::*;
        let cache = Cache::new("/Users/synoet/dev/imfs").unwrap();
        let file = cache.read("/Users/synoet/dev/imfs/src/lib.rs").unwrap();
        assert!(matches!(file, FileSystemItem::File(_)));
    }

    #[test]
    fn exists() {
        use super::*;
        let cache = Cache::new("/Users/synoet/dev/imfs").unwrap();
        let file = cache.exists("/Users/synoet/dev/imfs/src/lib.rs");
        assert!(file);
        let file = cache.exists("/Users/synoet/dev/imfs/src/lib.rs/does-not-exist");
        assert!(!file);
    }

    #[test]
    fn mkdir() {
        use super::*;
        let mut cache = Cache::new("/Users/synoet/dev/imfs").unwrap();
        let result = cache.mkdir("/Users/synoet/dev/imfs/src/test");
        assert!(result.is_ok());
        let result = cache.mkdir("/Users/synoet/dev/imfs/src/test");
        assert!(result.is_err());
    }

    #[test]
    fn write() {
        use super::*;
        let mut cache = Cache::new("/Users/synoet/dev/imfs").unwrap();
        let result = cache.write(
            "/Users/synoet/dev/imfs/src/test.txt",
            "test.txt",
            vec![1, 2, 3],
        );
        assert!(result.is_ok());
        let result = cache.write(
            "/Users/synoet/dev/imfs/src/test.txt",
            "test.txt",
            vec![1, 2, 3],
        );
        assert!(result.is_err());

        let file = cache.read("/Users/synoet/dev/imfs/src/test.txt").unwrap();
        assert!(matches!(file, FileSystemItem::File(_)));
    }

    #[test]
    fn rm() {
        use super::*;
        let mut cache = Cache::new("/Users/synoet/dev/imfs").unwrap();
        let result = cache.rm("/Users/synoet/dev/imfs/src");
        assert!(result.is_ok());
        let dir = cache.read("/Users/synoet/dev/imfs/src");
        assert!(dir.is_err());
        let file = cache.read("/Users/synoet/dev/imfs/src/lib.rs");

        assert!(file.is_err());
    }
}
