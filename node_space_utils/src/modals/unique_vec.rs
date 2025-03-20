pub struct UniqueVec<T>
where
    T: PartialEq,
{
    data: Vec<T>,
}

impl<T> UniqueVec<T>
where
    T: PartialEq,
{
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        if !self.data.contains(&value) {
            self.data.push(value);
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }
}

impl<T> IntoIterator for UniqueVec<T>
where
    T: PartialEq,
{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
