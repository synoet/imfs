use imfs::Cache;

fn main() {
    let mut cache = Cache::new("/Users/synoet/dev/imfs").unwrap();
    cache.read("/Users/synoet/dev/imfs/test.txt").unwrap();
}
