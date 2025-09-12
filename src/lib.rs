use std::{collections::HashMap, fmt::Debug, hash::Hash};

/// Return type of unique_inverse. Contains an vec with a unique-array and an inverses-array
#[derive(Debug, PartialEq)]
pub struct UniqueInverse<T>
where
    T: PartialEq,
{
    pub uniques: Vec<T>,
    pub inverses: Vec<usize>,
}

/// Computes the values and returns the uniques and an inverse array
/// # Example
/// ```
/// use unique_inverse::*;
///
/// let result = unique_inverse(vec![1,1,1]);
/// assert_eq!(result,UniqueInverse{uniques: vec![1],inverses: vec![0,0,0]});
/// ```
pub fn unique_inverse<T>(input: Vec<T>) -> UniqueInverse<T>
where
    T: Eq + Hash + Debug + Default + Clone,
{
    let mut unique_lookup: HashMap<T, usize> = HashMap::with_capacity(input.len());
    let mut inv = Vec::with_capacity(input.len());
    for item in input {
        let len = unique_lookup.len();
        let entry = unique_lookup.entry(item).or_insert(len);
        inv.push(*entry);
    }
    unique_lookup.shrink_to_fit();

    let mut uniques = vec![Default::default(); unique_lookup.len()];
    for (k, idx) in unique_lookup {
        uniques[idx] = k;
    }
    UniqueInverse {
        uniques: uniques,
        inverses: inv,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique() {
        let result = unique_inverse(vec![1, 2, 3, 4, 4, 3, 2, 1, 5]);
        assert_eq!(
            result,
            UniqueInverse {
                uniques: vec![1, 2, 3, 4, 5],
                inverses: vec![0, 1, 2, 3, 3, 2, 1, 0, 4]
            }
        );
    }
}
