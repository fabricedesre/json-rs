use collections::Vec;

// Poor man's replacement for a HashMap.
#[derive(Clone, Debug, PartialEq)]
pub struct HashMap<K, V> {
    data: Vec<(K, V)>,
}

impl<K, V> HashMap<K, V> {
    pub fn new() -> Self {
        HashMap { data: Vec::new() }
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.data.push((key, value));
    }
}

// impl<K, V> core::iter::Iterator for HashMap<K, V> {}