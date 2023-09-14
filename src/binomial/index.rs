use std::cmp::Ord;

#[derive(Debug)]
pub struct Index<T: Ord>(Vec<Value<T>>);

#[derive(Debug)]
pub struct Value<T: Ord> {
    offset: usize,
    value: T,
}

#[derive(Debug)]
pub enum IndexError<'a, T> {
    SimilarKeys,
    MissingKey(&'a T),
}

impl<T: Ord> Index<T> {
    pub fn from_keys(mut keys: Vec<T>) -> Self {
        keys.dedup();
        keys.sort_unstable();
        keys.reverse();
        let mut values = Vec::with_capacity(keys.len());
        let mut offset = 0;
        while let Some(value) = keys.pop() {
            values.push(Value { offset, value });
            offset += keys.len();
        }
        Index(values)
    }

    pub fn total_values(&self) -> usize {
        self.0.last().map(|v| v.offset).unwrap_or(0)
    }

    pub fn position<'b>(
        &self,
        key_a: &'b T,
        key_b: &'b T,
    ) -> Result<usize, IndexError<'b, T>> {
        if *key_a == *key_b {
            return Err(IndexError::SimilarKeys);
        }

        let (left, right) = if *key_a < *key_b {
            (key_a, key_b)
        } else {
            (key_b, key_a)
        };

        let start = self
            .0
            .binary_search_by(|probe| probe.value.cmp(left))
            .map_err(|_| IndexError::MissingKey(left))?;

        let end = self
            .0
            .binary_search_by(|probe| probe.value.cmp(right))
            .map_err(|_| IndexError::MissingKey(right))?;

        Ok(self.0[start].offset + end - start - 1)
    }

    pub fn iter_key_pairs(&self) -> IndexKeyPairIter<'_, T> {
        IndexKeyPairIter::new(self)
    }
}

#[derive(Debug)]
pub struct IndexKeyPairIter<'a, T: Ord> {
    index: &'a Index<T>,
    key: usize,
    walk: usize,
}

impl <'a, T: Ord> IndexKeyPairIter<'a, T> {
    fn new(index: &'a Index<T>) -> Self {
        Self {
            index,
            key: 0,
            walk: 1,
        }
    }
}

impl <'a, T: Ord> Iterator for IndexKeyPairIter<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.key + self.walk == self.index.0.len() {
            return None;
        }

        let left = &self.index.0[self.key].value;
        let right = &self.index.0[self.key + self.walk].value;

        if self.key + self.walk + 1 == self.index.0.len() {
            self.key += 1;
            self.walk = 1;
        } else {
            self.walk += 1;
        }

        Some((left, right))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn total_values() {
        let index: Index<i32> = Index::from_keys(vec![]);
        assert_eq!(0, index.total_values());

        let index = Index::from_keys(vec![1, 2, 3]);
        assert_eq!(3, index.total_values());

        let index = Index::from_keys(vec![1, 2, 3, 4]);
        assert_eq!(6, index.total_values());

        let index = Index::from_keys(vec![1, 2, 3, 4, 4]);
        assert_eq!(6, index.total_values());

        let index = Index::from_keys(vec![1, 2, 3, 4, 5]);
        assert_eq!(10, index.total_values());
    }

    #[test]
    fn position() {
        let index = Index::from_keys(vec![1, 2, 3]);
        assert_eq!(0, index.position(&1, &2).unwrap());
        assert_eq!(0, index.position(&2, &1).unwrap());
        assert_eq!(1, index.position(&1, &3).unwrap());
        assert_eq!(1, index.position(&3, &1).unwrap());
        assert_eq!(2, index.position(&2, &3).unwrap());
        assert_eq!(2, index.position(&3, &2).unwrap());

        let result = index.position(&3, &5);
        assert!(matches!(result, Err(IndexError::MissingKey(&5))));
        let result = index.position(&5, &1);
        assert!(matches!(result, Err(IndexError::MissingKey(&5))));

        let result = index.position(&1, &1);
        assert!(matches!(result, Err(IndexError::SimilarKeys)));
    }

    #[test]
    fn iter_key_pairs() {
        let index = Index::from_keys(vec![1, 2, 3, 4]);
        let mut iter = index.iter_key_pairs();
        assert_eq!((&1, &2), iter.next().unwrap());
        assert_eq!((&1, &3), iter.next().unwrap());
        assert_eq!((&1, &4), iter.next().unwrap());
        assert_eq!((&2, &3), iter.next().unwrap());
        assert_eq!((&2, &4), iter.next().unwrap());
        assert_eq!((&3, &4), iter.next().unwrap());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next());
    }
}
