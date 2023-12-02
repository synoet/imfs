use std::collections::HashMap;
use std::time::SystemTime;
use std::fs;
use std::path;
use std::time::{Duration, Instant};
use std::rc::Rc;

#[derive(Debug)]
#[warn(dead_code)]
pub struct File {
    pub name: String,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub size: u64,
    pub buffer: Vec<u8>,
    pub location: String,
    cache_modified: bool,
}

impl File {
    fn new (location: String) -> File {
        let metadata = fs::metadata(&location).unwrap();
        let buffer = fs::read(&location).unwrap();

        File {
            name: location.clone(),
            created: metadata.created().unwrap(),
            modified: metadata.modified().unwrap(),
            size: metadata.len(),
            buffer,
            location,
            cache_modified: false,
        }
    }
}

#[derive(Debug)]
pub struct Directory {
    pub location: String,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub items: HashMap<String, Rc<FileSystemItem>>,
    cache_modified: bool,
}

#[derive(Debug)]
pub enum FileSystemItem {
    File(File),
    Directory(Directory)
}

trait FileSystemItemTrait {
    fn location(&self) -> String;
}

impl FileSystemItemTrait for FileSystemItem {
    fn location(&self) -> String {
        match self {
            FileSystemItem::File(file) => file.location.clone(),
            FileSystemItem::Directory(dir) => dir.location.clone(),
        }
    }
}


pub struct Cache {
    pub root: Option<Directory>,
    pub host_location: String,
    pub location_map: HashMap<String, Rc<FileSystemItem>>,
}

impl Directory {
    pub fn new(location: String) -> Directory {
        Directory {
            location,
            created: SystemTime::now(),
            modified: SystemTime::now(),
            items: HashMap::new(),
            cache_modified: false,
        }
    }
}



// What should Cache support ?
// read (both file and dir)
// write (both file and dir)
// exists (both file and dir)
// copy (both file and dir)
// sync whole cache to user disk

impl Cache {
    pub fn new(host_location: String) -> Cache {
        let mut cache = Cache {
            root: None,
            host_location: host_location.clone(),
            location_map: HashMap::new(),
        };

        let mut dir = Directory::new(host_location.clone());

        cache._init_dir(&mut dir);

        cache.root = Some(dir);


        cache
    }

    fn _init_dir(&mut self, dir: &mut Directory ) {
        let mut items: HashMap<String, Rc<FileSystemItem>> = HashMap::new();
        let root = fs::read_dir(&dir.location).unwrap();

        for entry in root {
            let entry = entry.unwrap();
            let path = entry.path();
            let metadata = entry.metadata().unwrap();

            if metadata.is_dir() {
                let dir = FileSystemItem::Directory(Directory::new(path.to_str().unwrap().to_string()));
                let location = dir.location().clone();
                items.insert(dir.location(), Rc::new(dir));

                if let Some(dir_ref) = items.get(&location) {
                    self.location_map.insert(location, Rc::clone(dir_ref));
                }

            } else {
                let file = FileSystemItem::File(File::new(path.to_str().unwrap().to_string()));
                let location = file.location().clone();
                items.insert(file.location(), Rc::new(file));

                if let Some(file_ref) = items.get(&location) {
                    self.location_map.insert(location, Rc::clone(file_ref));
                }
            }
        }
        dir.items = items;
    }

    pub fn read(&self, location: String) -> Option<&FileSystemItem> {
        if let Some(item) = self.location_map.get(&location) {
            return Some(item);
        }

        None
    }
}

fn main() {
    let cache = Cache::new(String::from("/Users/synoet/Documents/"));
    let cache_start = Instant::now();
    cache.read(String::from("/Users/synoet/Documents/compare test"));
    let cache_end = cache_start.elapsed();
    println!("Cache took: {:?}", cache_end);

    let disk_start = Instant::now();
    let file = fs::read("/Users/synoet/Documents/compare test");
    let disk_end = disk_start.elapsed();

    println!("Disk took: {:?}", disk_end);

}
