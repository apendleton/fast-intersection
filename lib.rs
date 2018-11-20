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