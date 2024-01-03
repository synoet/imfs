use imfs::Cache;

fn main() {
    let mut cache = Cache::new("/Users/synoet/dev/imfs").unwrap();
    // cache.write("/Users/synoet/dev/imfs/test.txt", "test.txt", vec![1, 2, 3, 4, 5, 6]).unwrap();
    cache.rm("/Users/synoet/dev/imfs/test.txt").unwrap();
    cache.sync().unwrap();
}
