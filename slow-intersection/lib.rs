use std::cmp;
use std::collections::HashSet;

pub fn merge_intersection(set1: &[u32], set2: &[u32]) -> Vec<u32> {
    let mut it1 = set1.iter();
    let mut it2 = set2.iter();
    let mut out: Vec<u32> = Vec::with_capacity(cmp::min(set1.len(), set2.len()));

    let mut el1 = it1.next();
    let mut el2 = it2.next();
    loop {
        match (el1, el2) {
            (Some(val1), Some(val2)) => {
                if val1 < val2 {
                    el1 = it1.next();
                } else if val2 < val1 {
                    el2 = it2.next();
                } else {
                    out.push(*val1);
                    el1 = it1.next();
                    el2 = it2.next();
                }
            },
            _ => { break; }
        }
    }
    out
}

pub fn hashset_intersection(set1: &[u32], set2: &[u32]) -> Vec<u32> {
    let mut out: Vec<u32> = Vec::with_capacity(cmp::min(set1.len(), set2.len()));
    let hs: HashSet<u32> = set1.iter().cloned().collect();

    for item in set2.iter() {
        if hs.contains(item) {
            out.push(*item);
        }
    }
    out
}

#[test]
fn merge_test() {
    let a: Vec<u32> = vec![1, 2, 3, 4, 5];
    let b: Vec<u32> = vec![3, 5, 7, 9, 11];
    let inter = merge_intersection(&a, &b);
    assert_eq!(inter, vec![3, 5]);
}

#[test]
fn hash_test() {
    let a: Vec<u32> = vec![1, 2, 3, 4, 5];
    let b: Vec<u32> = vec![3, 5, 7, 9, 11];
    let inter = hashset_intersection(&a, &b);
    assert_eq!(inter, vec![3, 5]);
}