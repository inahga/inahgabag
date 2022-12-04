use std::collections::HashSet;
use std::hash::Hash;

pub fn intersection<A, B, T>(lists: B) -> Option<HashSet<T>>
where
    A: IntoIterator<Item = T>,
    B: IntoIterator<Item = A>,
    T: Eq + Hash,
{
    let mut iter = lists.into_iter();
    let mut ret = iter.next()?.into_iter().collect::<HashSet<_>>();
    for next in iter {
        ret = next.into_iter().filter(|e| ret.contains(e)).collect();
    }
    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersection() {
        assert!(
            intersection(vec![vec![1, 2, 3], vec![3, 4, 5], vec![3, 6, 7]]).unwrap()
                == HashSet::from([3])
        );
        assert!(intersection(vec![vec![1, 2, 3]]).unwrap() == HashSet::from([1, 2, 3]));
        assert!(intersection::<Vec<i32>, _, _>(vec![]) == None);
        assert!(
            intersection(vec![
                String::from("abc").chars(),
                String::from("ccc").chars()
            ])
            .unwrap()
                == HashSet::from(['c'])
        );
    }
}
