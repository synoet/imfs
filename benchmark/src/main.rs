use imfs::Cache;
use std::time::Instant;
use tabled::{Table, Tabled};


#[derive(Tabled)]
struct SimpleStat {
    name: String,
    init_time: String,
    write_time: String,
    read_time: String,
    mkdir_time: String,
    rm_time: String,
}


fn main() {
    let cache_init_timer = Instant::now();
    let mut cache = Cache::new("/Users/synoet/dev/imfs").unwrap();
    let cache_init_time = cache_init_timer.elapsed();

    let cache_read_timer = Instant::now();
    cache.read("/Users/synoet/dev/imfs/src/lib.rs").unwrap();
    let cache_read_time = cache_read_timer.elapsed();

    let cache_mkdir_timer = Instant::now();
    cache.mkdir("/Users/synoet/dev/imfs/src/test").unwrap();
    let cache_mkdir_time = cache_mkdir_timer.elapsed();

    let cache_write_timer = Instant::now();
    cache.write("/Users/synoet/dev/imfs/src/test/test.txt", "test.txt", "Hello World!".as_bytes().to_vec()).unwrap();
    let cache_write_time = cache_write_timer.elapsed();

    let cache_rm_timer = Instant::now();
    cache.rm("/Users/synoet/dev/imfs/src/test/test.txt").unwrap();
    let cache_rm_time = cache_rm_timer.elapsed();

    let fs_read_timer = Instant::now();
    std::fs::read("/Users/synoet/dev/imfs/src/lib.rs").unwrap();
    let fs_read_time = fs_read_timer.elapsed();

    let fs_mkdir_timer = Instant::now();
    std::fs::create_dir("/Users/synoet/dev/imfs/src/test").unwrap();
    let fs_mkdir_time = fs_mkdir_timer.elapsed();

    let fs_write_timer = Instant::now();
    std::fs::write("/Users/synoet/dev/imfs/src/test/test.txt", "Hello World!").unwrap();
    let fs_write_time = fs_write_timer.elapsed();

    let fs_rm_timer = Instant::now();
    std::fs::remove_file("/Users/synoet/dev/imfs/src/test/test.txt").unwrap();
    let fs_rm_time = fs_rm_timer.elapsed();

    std::fs::remove_dir("/Users/synoet/dev/imfs/src/test").unwrap();

    let stats = vec![
        SimpleStat {
            name: "imfs::Cache".to_string(),
            init_time: format!("{:?}", cache_init_time),
            write_time: format!("{:?}", cache_write_time),
            read_time: format!("{:?}", cache_read_time),
            mkdir_time: format!("{:?}", cache_mkdir_time),
            rm_time: format!("{:?}", cache_rm_time),
        },
        SimpleStat {
            name: "std::fs".to_string(),
            init_time: format!("0ms"),
            write_time: format!("{:?}", fs_write_time),
            read_time: format!("{:?}", fs_read_time),
            mkdir_time: format!("{:?}", fs_mkdir_time),
            rm_time: format!("{:?}", fs_rm_time),
        },
    ];

    let table = Table::new(stats).to_string();

    print!("{}", table);
}
