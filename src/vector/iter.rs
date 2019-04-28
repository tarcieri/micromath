use super::Vector;

/// Iterator over the components of an algebraic vector
pub struct Iter<'a, V>
where
    V: Vector,
{
    /// Reference to the original vector
    vector: &'a V,

    /// Iteration position within the vector
    position: usize,
}

impl<'a, V> Iter<'a, V>
where
    V: Vector,
{
    /// Create a new iterator over the vector's components
    pub(super) fn new(vector: &'a V) -> Self {
        Self {
            vector,
            position: 0,
        }
    }
}

impl<'a, V> Iterator for Iter<'a, V>
where
    V: Vector,
{
    type Item = V::Component;

    fn next(&mut self) -> Option<V::Component> {
        let item = self.vector.get(self.position);
        self.position += 1;
        item
    }
}
