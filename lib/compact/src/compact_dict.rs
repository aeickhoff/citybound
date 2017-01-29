use super::allocators::{Allocator, DefaultHeap};
use super::compact::Compact;
use super::compact_vec::CompactVec;

/// A simple linear-search key-value dictionary,
/// implemented using two `CompactVec`'s, one for keys, one for values.
///
/// The API loosely follows that of `std::collections::HashMap`.
/// Spilling behaviour using `Allocator` is equivalent to `CompactVec`.
pub struct CompactDict<K: Copy, V: Compact + Clone, A: Allocator = DefaultHeap> {
    keys: CompactVec<K, A>,
    values: CompactVec<V, A>,
}

impl<K: Eq + Copy, V: Compact + Clone, A: Allocator> CompactDict<K, V, A> {
    /// Create new, empty dictionary
    pub fn new() -> Self {
        CompactDict {
            keys: CompactVec::new(),
            values: CompactVec::new(),
        }
    }

    /// Amount of entries in the dictionary
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Is the dictionary empty?
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Look up the value for key `query`, if it exists
    pub fn get(&self, query: K) -> Option<&V> {
        for i in 0..self.keys.len() {
            if self.keys[i] == query {
                return Some(&self.values[i]);
            }
        }
        None
    }

    /// Lookup up the value for key `query`, if it exists, but also swap the entry
    /// to the beginning of the key/value vectors, so a repeated lookup for that item will be faster
    pub fn get_mru(&mut self, query: K) -> Option<&V> {
        for i in 0..self.keys.len() {
            if self.keys[i] == query {
                self.keys.swap(0, i);
                self.values.swap(0, i);
                return Some(&self.values[0]);
            }
        }
        None
    }

    /// Lookup up the value for key `query`, if it exists, but also swap the entry
    /// one index towards the beginning of the key/value vectors, so frequently repeated lookups
    /// for that item will be faster
    pub fn get_mfu(&mut self, query: K) -> Option<&V> {
        for i in 0..self.keys.len() {
            if self.keys[i] == query {
                if i > 0 {
                    self.keys.swap(i - 1, i);
                    self.values.swap(i - 1, i);
                    return Some(&self.values[i - 1]);
                } else {
                    return Some(&self.values[0]);
                }
            }
        }
        None
    }

    /// Does the dictionary contain a value for `query`?
    pub fn contains_key(&self, query: K) -> bool {
        self.keys.contains(&query)
    }

    /// Insert new value at key `query` and return the previous value at that key, if any existed
    pub fn insert(&mut self, query: K, new_value: V) -> Option<V> {
        for i in 0..self.keys.len() {
            if self.keys[i] == query {
                let old_val = self.values[i].clone();
                self.values[i] = new_value;
                return Some(old_val);
            }
        }
        self.keys.push(query);
        self.values.push(new_value);
        None
    }

    /// Remove value at key `query` and return it, if it existed
    pub fn remove(&mut self, query: K) -> Option<V> {
        for i in 0..self.keys.len() {
            if self.keys[i] == query {
                let old_val = self.values[i].clone();
                self.keys.remove(i);
                self.values.remove(i);
                return Some(old_val);
            }
        }
        None
    }

    /// Iterator over all keys in the dictionary
    pub fn keys(&self) -> ::std::slice::Iter<K> {
        self.keys.iter()
    }

    /// Iterator over all values in the dictionary
    pub fn values(&self) -> ::std::slice::Iter<V> {
        self.values.iter()
    }

    /// Iterator over mutable references to all values in the dictionary
    pub fn values_mut(&mut self) -> ::std::slice::IterMut<V> {
        self.values.iter_mut()
    }

    /// Iterator over all key-value pairs in the dictionary
    #[allow(needless_lifetimes)]
    pub fn pairs<'a>(&'a self) -> impl Iterator<Item = (&'a K, &'a V)> + Clone + 'a {
        self.keys().zip(self.values())
    }
}

impl<K: Eq + Copy, I: Compact, A1: Allocator, A2: Allocator> CompactDict<K, CompactVec<I, A1>, A2> {
    /// Push a value onto the `CompactVec` at the key `query`
    pub fn push_at(&mut self, query: K, item: I) {
        for i in 0..self.keys.len() {
            if self.keys[i] == query {
                self.values[i].push(item);
                return;
            }
        }
        self.keys.push(query);
        let mut vec = CompactVec::new();
        vec.push(item);
        self.values.push(vec);
    }

    /// Iterator over the `CompactVec` at the key `query`
    #[allow(needless_lifetimes)]
    pub fn get_iter<'a>(&'a self, query: K) -> impl Iterator<Item = &'a I> + 'a {
        self.get(query).into_iter().flat_map(|vec_in_option| vec_in_option.iter())
    }

    /// Remove the `CompactVec` at the key `query` and iterate over its elements (if it existed)
    #[allow(needless_lifetimes)]
    pub fn remove_iter<'a>(&'a mut self, query: K) -> impl Iterator<Item = I> + 'a {
        self.remove(query).into_iter().flat_map(|vec_in_option| vec_in_option.into_iter())
    }
}

impl<K: Copy, V: Compact + Clone, A: Allocator> Compact for CompactDict<K, V, A> {
    fn is_still_compact(&self) -> bool {
        self.keys.is_still_compact() && self.values.is_still_compact()
    }

    fn dynamic_size_bytes(&self) -> usize {
        self.keys.dynamic_size_bytes() + self.values.dynamic_size_bytes()
    }

    unsafe fn compact_from(&mut self, source: &Self, new_dynamic_part: *mut u8) {
        self.keys.compact_from(&source.keys, new_dynamic_part);
        self.values.compact_from(&source.values,
                                 new_dynamic_part.offset(self.keys.dynamic_size_bytes() as isize));
    }

    unsafe fn decompact(&self) -> CompactDict<K, V, A> {
        CompactDict {
            keys: self.keys.decompact(),
            values: self.values.decompact(),
        }
    }
}

impl<K: Copy, V: Compact + Clone, A: Allocator> Clone for CompactDict<K, V, A> {
    fn clone(&self) -> Self {
        CompactDict {
            keys: self.keys.clone(),
            values: self.values.clone(),
        }
    }
}

impl<K: Copy + Eq, V: Compact + Clone, A: Allocator> Default for CompactDict<K, V, A> {
    fn default() -> Self {
        CompactDict::new()
    }
}

impl<K: Copy + Eq, V: Compact + Clone, A: Allocator> ::std::iter::FromIterator<(K, V)>
    for CompactDict<K, V, A> {
    /// Construct a compact dictionary from an interator over key-value pairs
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut dict = Self::new();
        for (key, value) in iter {
            dict.insert(key, value);
        }
        dict
    }
}

impl<K: Copy + Eq, V: Compact + Clone, A: Allocator> ::std::iter::Extend<(K, V)>
    for CompactDict<K, V, A> {
    /// Extend a compact dictionary from an iterator over key-value pairs
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            self.insert(key, value);
        }
    }
}
