pub(crate) mod fnv {
    use std::hash::{BuildHasher, Hasher};

    const FNV_OFFSET_BASIC: u64 = 14695981039346656037;
    const FNV_PRIME: u64 = 1099511628211;

    pub struct FNVHasher {
        state: u64,
    }

    impl Hasher for FNVHasher {
        fn finish(&self) -> u64 {
            self.state
        }

        fn write(&mut self, bytes: &[u8]) {
            for byte in bytes {
                self.state ^= *byte as u64;
                self.state = self.state.wrapping_mul(FNV_PRIME);
            }
        }
    }

    #[derive(Default)]
    pub struct FNVHashBuilder;

    impl BuildHasher for FNVHashBuilder {
        type Hasher = FNVHasher;

        fn build_hasher(&self) -> Self::Hasher {
            FNVHasher {
                state: FNV_OFFSET_BASIC,
            }
        }
    }
}

use std::collections::{
    hash_map::{IntoIter, Iter, IterMut},
    HashMap,
};
use std::iter::IntoIterator;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Default)]
pub struct FastMap<K, V>(HashMap<K, V, fnv::FNVHashBuilder>);

impl<K, V> FastMap<K, V> {
    pub fn new() -> Self {
        Map(HashMap::with_hasher(fnv::FNVHashBuilder))
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Map(HashMap::with_capacity_and_hasher(capacity, fnv::FNVHashBuilder))
    }
}

impl<K, V> Deref for FastMap<K, V> {
    type Target = HashMap<K, V, fnv::FNVHashBuilder>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> DerefMut for FastMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V> IntoIterator for FastMap<K, V> {
    type IntoIter = IntoIter<K, V>;
    type Item = (K, V);
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a FastMap<K, V> {
    type IntoIter = Iter<'a, K, V>;
    type Item = (&'a K, &'a V);
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut FastMap<K, V> {
    type IntoIter = IterMut<'a, K, V>;
    type Item = (&'a K, &'a mut V);
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
