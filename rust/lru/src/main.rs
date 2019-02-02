use std::collections::HashMap;
fn main() {
    let mut cache = LruCache::new(2);
    println!("{:?}", cache);
    cache.set("a", 1);
    println!("{:?}", cache.get("a"));
    println!("{:?}", cache.get("b"));
    cache.set("b", 2);
    println!("{:?}", cache.get("b"));
}

#[derive(Debug)]
struct LruCache<'a> {
    size: i32,
    cache: HashMap<&'a str, i32>
}

impl<'a> LruCache<'a> {
    pub fn new(max: i32) -> Self {
        LruCache{size: max, cache: HashMap::new()}
    }
    pub fn set(&mut self, k: &'a str, v: i32) -> Option<i32>{
        self.cache.insert(k, v)
    }
    pub fn get(&mut self, k: &'a str) -> Option<&i32>{
        self.cache.get(k)
    }
}

#[test]
fn test_set_and_get() {
    let mut cache = LruCache::new(2);
    cache.set("a", 1);
    assert_eq!(cache.get("a"), Some(&1));
    assert_eq!(cache.get("b"), None);
    cache.set("b", 2);
    assert_eq!(cache.get("b"), Some(&2));
    cache.set("a", 3);
    assert_eq!(cache.get("a"), Some(&3));
}