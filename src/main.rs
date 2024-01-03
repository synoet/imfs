mod htm;
use path::Component;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::path;
use std::rc::Rc;
use std::time::Instant;
use std::time::SystemTime;

#[derive(Debug, Clone)]
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
    fn new(location: String) -> File {
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

#[derive(Debug, Clone)]
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
    Directory(Directory),
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

trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for path::Component<'_> {
    fn to_string(&self) -> String {
        return self.as_os_str().to_str().unwrap().to_string();
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

    pub fn set_modified(&mut self) {
        self.cache_modified = true;
    }
}

// What should Cache support ?
// -[x] read (both file and dir)
// -[ ] write (file)
// -[x] mkdir (dir)
// -[x] exists (both file and dir)
// -[ ] copy (both file and dir)
// -[ ] sync whole cache to user disk

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

    fn _init_dir(&mut self, dir: &mut Directory) {
        let mut items: HashMap<String, Rc<FileSystemItem>> = HashMap::new();
        let root = fs::read_dir(&dir.location).unwrap();

        for entry in root {
            let entry = entry.unwrap();
            let path = entry.path();
            let metadata = entry.metadata().unwrap();

            if metadata.is_dir() {
                let dir =
                    FileSystemItem::Directory(Directory::new(path.to_str().unwrap().to_string()));
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

    pub fn exists(&self, location: String) -> bool {
        self.location_map.contains_key(&location)
    }

    pub fn is_dir(&self, location: String) -> bool {
        if let Some(item) = self.location_map.get(&location) {
            if let FileSystemItem::Directory(_) = item.as_ref() {
                return true;
            }
            return false;
        }

        panic!("location does not exist");
    }

    fn within(&self, location: String) -> bool {
        location.starts_with(&self.host_location)
    }

    //TODO: this is kind of wonky
    pub fn mkdir(&mut self, location: String) {
        if !self.within(location.clone()) {
            panic!("location is not within cache");
        }

        let trimmed_location = location.replace(&self.host_location, "");
        let location_path = path::Path::new(&trimmed_location);
        let mut curr_location = self.root.as_ref().unwrap().location.clone();

        let components = location_path.components();
        for component in components.clone() {
            let formatted_component = component.as_os_str().to_str().unwrap().to_string();
            let target_dir = format!("{}{}", curr_location, formatted_component);

            if !self.exists(target_dir.clone()) {
                let mut raw_dir = Directory::new(target_dir.clone());
                raw_dir.set_modified();
                let dir = FileSystemItem::Directory(raw_dir);
                self.root
                    .as_mut()
                    .unwrap()
                    .items
                    .insert(target_dir.clone(), Rc::new(dir));
                self.location_map.insert(
                    target_dir.clone(),
                    Rc::clone(self.root.as_ref().unwrap().items.get(&target_dir).unwrap()),
                );
            }

            curr_location.push_str(format!("{}/", &formatted_component).as_str());
        }
    }

    pub fn write(&mut self, location: String, buffer: Vec<u8>) {
        if !self.within(location.clone()) {
            panic!("location is not within cache");
        }

        let trimmed_location = location.replace(&self.host_location, "");
        let location_path = path::Path::new(&trimmed_location);
        let file_to_write = location_path.components().last().unwrap();
        let file_to_write_dir = location.replace(file_to_write.as_os_str().to_str().unwrap(), "");

        if !self.exists(file_to_write_dir.clone()) {
            panic!("parent directory does not exist");
        }

        if !self.is_dir(file_to_write_dir) {
            panic!("the file does not live within a directory");
        }

        let file = FileSystemItem::File(File {
            name: file_to_write.as_os_str().to_str().unwrap().to_string(),
            created: SystemTime::now(),
            modified: SystemTime::now(),
            size: buffer.len() as u64,
            buffer,
            location: location.clone(),
            cache_modified: true,
        });
    }
}

fn main() {
    let mut cache = Cache::new(String::from("/Users/synoet/Documents/"));
    // let cache_start = Instant::now();
    // // cache.read(String::from("/Users/synoet/Documents"));
    // cache.exists(String::from("/Users/synoet/Documents"));
    // let cache_end = cache_start.elapsed();
    // println!("Cache took: {:?}", cache_end);

    // let disk_start = Instant::now();
    // // let file = fs::read("/Users/synoet/Documents");
    // fs::metadata("/Users/synoet/Documents").unwrap();

    // let disk_end = disk_start.elapsed();

    // cache.mkdir(String::from("/Users/synoet/Documents/backup_test/bingo"));
    cache.write(
        String::from("/Users/synoet/Documents/backup_test/bingo"),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
    );

    // println!("Disk took: {:?}", disk_end);
}
