// for demo purposes
struct Alist<K, V>(Vec<(K, V)>); // newtype pattern

impl<K, V> Alist<K, V> {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn insert(&mut self, key: K, value: V) {
        self.0.push((key, value));
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn into_iter(self) -> std::vec::IntoIter<(K, V)> {
        self.0.into_iter()
    }
}

use std::ops::Deref;

impl<K, V> Deref for Alist<K, V> {
    type Target = [(K, V)];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

fn main() {
    let mut l = Alist::new();
    for (x, y) in [(1, 2), (3, 4), (5, 6)] {
        l.insert(x, y);
    }

    let mut iter = l.iter();
    while let Some((k, v)) = iter.next() {
        println!("{k}: {v}");
    }
}
