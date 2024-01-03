# imfs (In-Memory File System)
**imfs** is a Rust library that provides a virtual wrapper for file system operations. It allows for the storage and manipulation of file system representations in memory, enabling fast and efficient operations. This library is particularly useful in scenarios where applications can handle longer startup and shutdown times but require rapid file operations during runtime.
### Features 
- In-memory caching of file system structures.
- Support for common file operations (read, write, create directories, copy, check existence).
- Ability to synchronize changes back to the disk.

### Basic Stats

This is by no means a good benchmark, just to show what kind of performance gain you get.

+-------------+-------------+------------+-----------+------------+-----------+
| name        | init_time   | write_time | read_time | mkdir_time | rm_time   |
+-------------+-------------+------------+-----------+------------+-----------+
| imfs::Cache | 82.672083ms | 2.333µs    | 1.791µs   | 3.084µs    | 1.833µs   |
+-------------+-------------+------------+-----------+------------+-----------+
| std::fs     | 0ms         | 118.667µs  | 5.958µs   | 61.875µs   | 301.958µs |
+-------------+-------------+------------+-----------+------------+-----------+%



### Usage

Reading a directory into the cache
```rust 
use imfs::Cache;

fn main() {
    let mut cache = Cache::new("/Users/synoet/dev/project").unwrap();
}
```

Reading a file from the cache
```rust
use imfs::Cache;

fn main() {
    let mut cache = Cache::new("/Users/synoet/dev/project").unwrap();
    let file = cache.read("/Users/synoet/dev/imfs/project/test.txt").unwrap();
}
```

Writing a file to the cache
```rust
use imfs::Cache;

fn main() {
    let mut cache = Cache::new("/Users/synoet/dev/project").unwrap();
    cache.write("/Users/synoet/dev/imfs/project/test.txt", "text.txt", vec![0,0,0,0]);
}
```

Making a new directory
```rust
use imfs::Cache;

fn main() {
    let mut cache = Cache::new("/Users/synoet/dev/project").unwrap();
    cache.mkdir("/Users/synoet/dev/project/dir")

}
```

Checking if a file / dir exist
```rust
use imfs::Cache;

fn main() {
    let mut cache = Cache::new("/Users/synoet/dev/project").unwrap();
    let does_exist: bool = cache.exists("/Users/synoet/dev/project/dir")

}
```

Deleting a file / dir
```rust
use imfs::Cache;

fn main() {
    let mut cache = Cache::new("/Users/synoet/dev/project").unwrap();
    cache.rm("/Users/synoet/dev/project/dir")

}
```

Syncing changes back to the disk


This will sync any changes made to the virtual cache back to it's original location on the file system
```rust
use imfs::Cache;

fn main() {
    let mut cache = Cache::new("/Users/synoet/dev/project").unwrap();
    cache.sync()
}
```
