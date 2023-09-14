mod index;
mod store;

pub use index::Index;
pub use index::IndexError;
pub use store::Store;
use std::cmp::Ord;

pub type Lookup<K, V> = Storage<K, V, Vec<V>>;

#[derive(Debug)]
pub struct Storage<KEY, VAL, STORE>
where
    KEY: Ord,
    VAL: Clone,
    STORE: Store<Type = VAL>
{
    index: Index<KEY>,
    store: STORE,
}

impl<K: Ord, V: Clone, S: Store<Type = V>> Storage<K, V, S> {
    pub fn from_keys(keys: Vec<K>, default: V) -> Self {
        let index = Index::from_keys(keys);
        let store = S::new(index.total_values(), default);
        Self { index, store }
    }

    pub fn from_keys_and_strategy<F>(keys: Vec<K>, mut func: F) -> Self
    where
        F: FnMut(&K, &K) -> V
    {
        let index = Index::from_keys(keys);
        let mut store = S::with_capacity(index.total_values());
        for (left, right) in index.iter_key_pairs() {
            store.push(func(left, right));
        }
        Self { index, store }
    }

    pub fn from_raw_parts(index: Index<K>, store: S) -> Self {
        Self { index, store }
    }

    pub fn into_raw_parts(self) -> (Index<K>, S) {
        (self.index, self.store)
    }

    pub fn get<'a, 'b>(
        &'a self,
        key_a: &'b K,
        key_b: &'b K
    ) -> Result<&'a V, IndexError<'b, K>> {
        let position = self.index.position(key_a, key_b)?;
        Ok(self.store.ref_at(position))
    }

    pub fn get_mut<'a, 'b>(
        &'a mut self,
        key_a: &'b K,
        key_b: &'b K,
    ) -> Result<&'a mut V, IndexError<'b, K>> {
        let position = self.index.position(key_a, key_b)?;
        Ok(self.store.mut_ref_at(position))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn from_keys() {
        let store = Lookup::from_keys(vec![1, 2, 3], 42);
        assert_eq!(&42, store.get(&1, &2).unwrap());

        let result = store.get(&5, &1);
        assert!(matches!(result, Err(IndexError::MissingKey(&5))));

        let result = store.get(&1, &1);
        assert!(matches!(result, Err(IndexError::SimilarKeys)));
    }

    #[test]
    fn from_keys_and_strategy() {
        let store = Lookup::from_keys_and_strategy(
            vec![1, 2, 3, 4, 5],
            |a, b| { *a + *b }
        );

        assert_eq!(&6, store.get(&1, &5).unwrap());
        assert_eq!(&9, store.get(&4, &5).unwrap());
    }

    #[test]
    fn get_mut() {
        let mut store = Lookup::from_keys(vec![1, 2, 3], 0);
        assert_eq!(&0, store.get(&1, &2).unwrap());
        *store.get_mut(&2, &1).unwrap() = 1;
        assert_eq!(&1, store.get(&1, &2).unwrap());
    }
}
