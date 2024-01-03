use imfs::Cache;

fn main() {
    let mut cache = Cache::new("/Users/synoet/dev/imfs").unwrap();
    cache.rm("/Users/synoet/dev/imfs/src");
}
