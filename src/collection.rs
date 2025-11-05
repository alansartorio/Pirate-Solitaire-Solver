use std::collections::{BTreeMap, btree_map::Entry};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BTreeMultiSet<T> {
    map: BTreeMap<T, usize>,
}

impl<T> Default for BTreeMultiSet<T> {
    fn default() -> Self {
        Self {
            map: BTreeMap::default(),
        }
    }
}

impl<T, const N: usize> From<[T; N]> for BTreeMultiSet<T>
where
    T: Ord + Clone,
{
    fn from(value: [T; N]) -> Self {
        Self::from_iter(value)
    }
}

impl<A> FromIterator<A> for BTreeMultiSet<A>
where
    A: Ord + Clone,
{
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut col = BTreeMultiSet::default();
        iter.into_iter().for_each(|value| col.insert(value));
        col
    }
}

impl<T> BTreeMultiSet<T>
where
    T: Ord + Clone,
{
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.map
            .iter()
            .flat_map(|(value, count)| std::iter::repeat_n(value, *count))
    }

    pub fn take(&mut self, value: &T) -> Option<T> {
        match self.map.entry(value.clone()) {
            Entry::Vacant(_) => None,
            Entry::Occupied(mut occupied_entry) => {
                *occupied_entry.get_mut() -= 1;
                if *occupied_entry.get() == 0 {
                    occupied_entry.remove();
                }
                Some(value.clone())
            }
        }
    }

    pub fn remove(&mut self, value: &T) -> Option<T> {
        self.take(value)
    }

    pub fn insert(&mut self, value: T) {
        match self.map.entry(value) {
            Entry::Vacant(entry) => {
                entry.insert(1);
            }
            Entry::Occupied(mut occupied_entry) => {
                *occupied_entry.get_mut() += 1;
            }
        }
    }
}
