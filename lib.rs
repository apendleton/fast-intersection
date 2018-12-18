extern crate simdintersection_sys;
use std::cmp;

pub fn simd_intersection(set1: &[u32], set2: &[u32]) -> Vec<u32> {
    unsafe {
        let len1 = set1.len();
        let len2 = set2.len();
        let pset1 = set1.as_ptr();
        let pset2 = set2.as_ptr();

        let maxlen = cmp::min(len1, len2);
        let mut dst = Vec::with_capacity(maxlen);
        let pdst = dst.as_mut_ptr();

        let dstlen = simdintersection_sys::_ZN18SIMDCompressionLib16SIMDintersectionEPKjmS1_mPj(
            pset1,
            len1,
            pset2,
            len2,
            pdst
        );
        dst.set_len(dstlen);
        dst
    }
}

#[test]
fn simple_intersection() {
    let a: Vec<u32> = vec![1, 2, 3, 4, 5];
    let b: Vec<u32> = vec![3, 5, 7, 9, 11];
    let inter = simd_intersection(&a, &b);
    assert_eq!(inter, vec![3, 5]);
}

#[cfg(test)]
mod fuzz_tests {
    extern crate slow_intersection;
    extern crate rand;
    use fuzz_tests::rand::distributions::{Distribution, Uniform};

    use std::collections::HashSet;
    use fuzz_tests::slow_intersection::*;
    use super::*;

    #[test]
    #[ignore]
    fn rand_intersections() {
        let values = Uniform::from(1..10000);
        let lengths = Uniform::from(100..1000);
        let mut rng = rand::thread_rng();

        for _ in 0..10000 {
            let mut s1: HashSet<u32> = HashSet::new();
            let l1 = lengths.sample(&mut rng);
            let mut s2: HashSet<u32> = HashSet::new();
            let l2 = lengths.sample(&mut rng);

            while s1.len() < l1 {
                s1.insert(values.sample(&mut rng));
            }
            while s2.len() < l2 {
                s2.insert(values.sample(&mut rng));
            }

            let mut s1: Vec<u32> = s1.iter().cloned().collect();
            s1.sort();
            let mut s2: Vec<u32> = s2.iter().cloned().collect();
            s2.sort();

            let simd = simd_intersection(&s1, &s2);
            let merge = merge_intersection(&s1, &s2);
            let hash = hashset_intersection(&s1, &s2);

            assert_eq!(simd, merge);
            assert_eq!(simd, hash);
        }
    }
}