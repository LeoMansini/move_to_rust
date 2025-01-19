
use std::sync::LazyLock;
use std::collections::HashMap;
use std::hash::Hash;


pub struct IdGetter {
    current_id: std::sync::Mutex<u8>,
}

impl IdGetter {
    pub fn new() -> Self {
        IdGetter {
            current_id: std::sync::Mutex::new(0),
        }
    }

    pub fn get_new_id(&self) -> u8 {
        let mut id = self.current_id.lock().unwrap();
        *id += 1;
        *id
    }
}

// Use LazyLock to initialize ID_GETTER
pub static ID_GETTER: LazyLock<IdGetter> = LazyLock::new(|| IdGetter::new());

trait Key: Eq + Hash {}
impl<T> Key for T where T: Eq + Hash {}

pub struct Table<K: Key, V> {
    id: u8,
    map: HashMap<K, V>,
    size: u8,
}

pub fn new<K: Key, V>() -> Table<K, V> {
    Table{
        id: ID_GETTER.get_new_id(),
        map: HashMap::new(),
        size: 0
    }
}

pub fn add<K: Key,V>(t: &mut Table<K, V>, key: K, value: V) -> () {
    if contains(t, &key) {
        panic!("Key already exists in Table");
    }
    t.map.insert(key, value);
    t.size = t.size + 1;
}

pub fn borrow<'a, K: Key, V>(t: &'a Table<K, V>, key: &'a K) -> &'a V {
    if !contains(t, &key) {
        panic!("Key missing from Table");
    }
    t.map.get(key).unwrap()
}

pub fn borrow_mut<'a, K: Key, V>(t: &'a mut Table<K, V>, key: &'a K) -> &'a mut V {
    if !contains(t, &key) {
        panic!("Key missing from Table");
    }
    t.map.get_mut(key).unwrap()
}

pub fn remove<'a, K: Key, V>(t: &'a mut Table<K, V>, key: &'a K) -> V {
    if !contains(t, &key) {
        panic!("Key missing from Table");
    }
    let value: V = t.map.remove(key).unwrap();
    t.size = t.size - 1;
    value
}

pub fn contains<K: Key, V>(t: &Table<K, V>, key: &K) -> bool {
    t.map.contains_key(key)
}

pub fn length<K: Key, V>(t: &Table<K, V>) -> u8 {
    t.size
}

pub fn is_empty<K: Key, V>(t: &Table<K, V>) -> bool {
    t.size == 0
}

pub fn destroy_empty<K: Key, V>(t: &Table<K, V>) -> () {
    assert!(is_empty(t));
}

pub fn drop<K: Key, V>(t: &Table<K, V>) -> () {
}