//! Permutation module. Naive generic implementation of permutations for Vec<T>.
//! Use with caution, this might use a lot of resources.

/// Permutations struct, holding all possible permutations.
#[derive(Debug)]
pub struct Permutations<T>(Vec<Vec<T>>);

/// Holds the internal state for the Iterator.
pub struct PermIter<'a, T> {
    perms: &'a Vec<Vec<T>>,
    curr: usize,
}

impl<T: Clone> From<Vec<T>> for Permutations<T> {
    fn from(value: Vec<T>) -> Self {
        let mut all_perms = Vec::new();
        Permutations::all_perms(value.len(), value, &mut all_perms);
        Self(all_perms)
    }
}

impl<T: Clone> Permutations<T> {
    /// Private method to generate all permutations from a given Vec.
    fn all_perms(n: usize, mut vec: Vec<T>, acc: &mut Vec<Vec<T>>) {
        let mut indexes = vec![0; n];

        acc.push(vec.clone());

        let mut i = 0;
        loop {
            if i >= n {
                break;
            }

            if indexes[i] < i {
                let idx = if i % 2 == 0 { 0 } else { indexes[i] };
                vec.swap(idx, i);
                acc.push(vec.clone());
                indexes[i] += 1;
                i = 0;
            } else {
                indexes[i] = 0;
                i += 1;
            }
        }
    }

    /// Creates a new Iterator on permutations.
    pub fn iter(&self) -> PermIter<'_, T> {
        PermIter {
            perms: &self.0,
            curr: 0,
        }
    }
}

impl<'a, T> Iterator for PermIter<'a, T> {
    type Item = &'a Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr >= self.perms.len() {
            None
        } else {
            let elem = &self.perms[self.curr];
            self.curr += 1;
            Some(elem)
        }
    }
}
