pub trait Store {
    type Type: Clone;

    fn with_capacity(capacity: usize) -> Self;

    fn ref_at(&self, position: usize) -> &Self::Type;

    fn mut_ref_at(&mut self, position: usize) -> &mut Self::Type;

    fn push(&mut self, value: Self::Type);

    fn new(size: usize, default: Self::Type) -> Self;
}

impl <V: Clone> Store for Vec<V> {
    type Type = V;

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity(capacity)
    }

    fn ref_at(&self, position: usize) -> &Self::Type {
        &self[position]
    }

    fn mut_ref_at(&mut self, position: usize) -> &mut Self::Type {
        &mut self[position]
    }

    fn push(&mut self, value: Self::Type) {
        self.push(value);
    }

    fn new(size: usize, default: Self::Type) -> Self {
        vec![default; size]
    }
}
