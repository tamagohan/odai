use std::collections::HashMap;
use std::hash::Hash;
use std::cmp::Eq;

fn main() {
    let mut cache = LruCache::new(2);
    println!("{:?}", cache);
    cache.set(&100, 1);
    println!("{:?}", cache.get(&100));
    println!("{:?}", cache.get(&101));
    cache.set(&101, 2);
    println!("{:?}", cache.get(&101));
}

#[derive(Debug)]
struct LruCache<'a, K, V>
    where K: Sized + Eq + Hash
{
    size: i32,
    cache: HashMap<&'a K, V>
}

impl<'a, K, V> LruCache<'a, K, V>
    where K: Sized + Eq + Hash
{
    pub fn new(max: i32) -> Self {
        LruCache{size: max, cache: HashMap::new()}
    }
    pub fn set(&mut self, k: &'a K, v: V) -> Option<V>{
        self.cache.insert(k, v)
    }
    pub fn get(&mut self, k: &'a K) -> Option<&V>{
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