use std::collections::HashMap;
use std::hash::Hash;
use crate::cache::Cache;

/// # LRU Cache - Least Recently Used
///
/// **entries** contains the cache entries.
/// **map** contains indexes of entries in the entries vector.
/// **first** and last are indexes of the first and last entries.
/// **max_size** is the maximum number of entries in the cache.
///
/// **How it works**:
/// - When a key is added to the cache, it is moved to the front.
/// - When a key is accessed, it is moved to the front.
/// - When the cache is full, the last entry is removed.
///
/// **Exemple**:
/// ```
///use lru_cache::cache::Cache;
///use lru_cache::lru_cache::LRUCache;
///
/// let mut cache = LRUCache::with_capacity(3); // Create a cache with a capacity of 3
/// cache.put(1, "A"); // Add a key-value pair to the cache
/// cache.put(2, "B");
/// cache.put(3, "C");
///
/// assert_eq!(cache.get(&1), Some(&"A"));
/// assert_eq!(cache.get(&2), Some(&"B"));
/// assert_eq!(cache.get(&3), Some(&"C"));
///
pub struct LRUCache<K, V> {
    entries: Vec<Entry<K, V>>,
    map: HashMap<K, usize>, // Clé -> index
    first: Option<usize>,
    last: Option<usize>,
    max_size: usize,
}

///
/// Cache entry
///
/// Contains key-value and next and previous entry indexes.
///
struct Entry<K, V> {
    key: K,
    value: Option<V>,
    prev: Option<usize>,
    next: Option<usize>,
}

impl<K, V> Cache<K, V> for LRUCache<K, V>
where
    K: Hash + Eq + Clone,
{
    fn with_capacity(max_size: usize) -> Self {
        Self {
            entries: Vec::with_capacity(max_size),
            map: HashMap::with_capacity(max_size),
            first: None,
            last: None,
            max_size,
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.get(key).is_some() {
            let index = *self.map.get(key).unwrap();

            self._move_to_front(index);

            self.entries[index].value.as_ref()
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) {
        if self.map.get(&key).is_some() {
            let index = *self.map.get(&key).unwrap();
            self.entries[index].value = Some(value);
            self._move_to_front(index);
            return;
        }

        let new_entry = Entry {
            key: key.clone(),
            value: Some(value),
            prev: None,
            next: self.first,
        };
        let new_index = self.entries.len();

        if self.entries.len() >= self.max_size {
            self._remove_last();
        }

        self.entries.push(new_entry);
        self.map.insert(key, new_index);

        match self.first {
            None => {
                self.first = Some(new_index);
                self.last = Some(new_index);
            }
            Some(old_first) => {
                self.first = Some(new_index);
                self.entries[new_index].next = Some(old_first);
                self.entries[old_first].prev = Some(new_index);
            }
        }
    }

    fn invalidate(&mut self, key: &K) {
        if self.map.get(key).is_some() {
            let index = *self.map.get(key).unwrap();
            let prev = self.entries[index].prev;
            let next = self.entries[index].next;

            if Some(index) == self.first {
                self.first = next;
            }

            if Some(index) == self.last {
                self.last = prev;
            }

            if prev.is_some() {
                self.entries[prev.unwrap()].next = next;
            }

            if next.is_some() {
                self.entries[next.unwrap()].prev = prev;
            }

            self.map.remove(key);
            self.entries[index].value = None;
        }
    }
}

impl <K, V> LRUCache<K, V>
where
    K: Hash + Eq + Clone,
{
    fn _remove_last(&mut self) {
        if self.last.is_some() {
            let last_index = self.last.unwrap();

            let last_key = &self.entries[last_index].key;
            self.map.remove(last_key);

            self.last = self.entries[last_index].prev;
            if self.last.is_some() {
                let new_last = self.last.unwrap();
                self.entries[new_last].next = None;
            } else {
                self.first = None;
            }
        }
    }

    fn _move_to_front(&mut self, index: usize) {
        if Some(index) == self.first {
            return;
        }

        let prev = self.entries[index].prev;
        let next = self.entries[index].next;

        if prev.is_some() {
            self.entries[prev.unwrap()].next = next;
        }

        if next.is_some() {
            self.entries[next.unwrap()].prev = prev;
        }

        if Some(index) == self.last {
            self.last = prev;
        }

        if self.first.is_some() {
            let old_first = self.first.unwrap();
            self.entries[old_first].prev = Some(index);
        }

        self.entries[index].prev = None;
        self.entries[index].next = self.first;
        self.first = Some(index);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_put() {
        let mut cache = LRUCache::with_capacity(3);
        cache.put("A", String::from("A"));
        cache.put("B", String::from("B"));
        cache.put("C", String::from("C"));

        assert_eq!(cache.get(&"A"), Some(&String::from("A")));
        assert_eq!(cache.get(&"B"), Some(&String::from("B")));
        assert_eq!(cache.get(&"C"), Some(&String::from("C")));
    }

    #[test]
    fn test_capacity() {
        let mut cache = LRUCache::with_capacity(3);
        cache.put("A", String::from("A"));
        cache.put("B", String::from("B"));
        cache.put("C", String::from("C"));
        cache.put("D", String::from("D"));

        assert_eq!(cache.get(&"A"), None);
        assert_eq!(cache.get(&"B"), Some(&String::from("B")));
        assert_eq!(cache.get(&"C"), Some(&String::from("C")));
        assert_eq!(cache.get(&"D"), Some(&String::from("D")));
    }

    #[test]
    fn test_invalidate() {
        let mut cache = LRUCache::with_capacity(3);
        cache.put("A", String::from("A"));
        cache.put("B", String::from("B"));

        cache.invalidate(&"A");
        assert_eq!(cache.get(&"A"), None);
        assert_eq!(cache.get(&"B"), Some(&String::from("B")));
    }
}