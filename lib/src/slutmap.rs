/// Key of a [`Slutmap`].
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Key(usize);

/// A shameful [slotmap](https://docs.rs/slotmap/latest/slotmap).
///
/// Never reuses slots not deallocates.
/// It's both lame and great: you never get an old item when you `get`, just `None`.
#[derive(Clone, Debug)]
pub struct Slutmap<T> {
    items: Vec<Option<T>>,
    len: usize,
}

impl<T> Default for Slutmap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Slutmap<T> {
    pub fn new() -> Self {
        Self {
            items: Default::default(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, key: Key) -> Option<&T> {
        self.items.get(key.0).and_then(|item| item.as_ref())
    }

    pub fn get_mut(&mut self, key: Key) -> Option<&mut T> {
        self.items.get_mut(key.0).and_then(|item| item.as_mut())
    }

    pub fn insert(&mut self, item: T) -> Key {
        let key = Key(self.items.len());

        self.len += 1;
        self.items.push(Some(item));

        key
    }

    pub fn remove(&mut self, key: Key) -> Option<T> {
        self.len -= 1;
        std::mem::take(self.items.get_mut(key.0).expect("Unreachable: invalid key"))
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }
}
