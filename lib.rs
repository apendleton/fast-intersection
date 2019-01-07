extern crate simdintersection_sys;
extern crate streamvbyte_sys;
use std::cmp;

// SIMD Intersection
pub fn simd_intersection(set1: &[u32], set2: &[u32]) -> Vec<u32> {
    unsafe {
        let len1 = set1.len();
        let len2 = set2.len();
        let pset1 = set1.as_ptr();
        let pset2 = set2.as_ptr();

        let maxlen = cmp::min(len1, len2);
        let mut dst = Vec::with_capacity(maxlen);
        let pdst = dst.as_mut_ptr();

        let dstlen = simdintersection_sys::SIMDintersection(
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

pub fn simd_intersection_avx2(set1: &[u32], set2: &[u32]) -> Vec<u32> {
    unsafe {
        let len1 = set1.len();
        let len2 = set2.len();
        let pset1 = set1.as_ptr();
        let pset2 = set2.as_ptr();

        let maxlen = cmp::min(len1, len2);
        let mut dst = Vec::with_capacity(maxlen);
        let pdst = dst.as_mut_ptr();

        let dstlen = simdintersection_sys::SIMDintersection_avx2(
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
    let inter2 = simd_intersection_avx2(&a, &b);
    assert_eq!(inter, vec![3, 5]);
    assert_eq!(inter, inter2);
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
            let simd_avx2 = simd_intersection_avx2(&s1, &s2);
            let merge = merge_intersection(&s1, &s2);
            let hash = hashset_intersection(&s1, &s2);

            assert_eq!(simd, merge);
            assert_eq!(simd, simd_avx2);
            assert_eq!(simd, hash);
        }
    }
}

// streambyte
pub fn streamvbyte_encode(in_: &[u32]) -> Vec<u8> {
    unsafe {
        let len = in_.len();
        let pin = in_.as_ptr();

        let maxlen = streamvbyte_sys::streamvbyte_max_compressedbytes(len);
        let mut dst = Vec::with_capacity(maxlen);
        let pdst = dst.as_mut_ptr();

        let dstlen = streamvbyte_sys::streamvbyte_encode(
            pin,
            len as u32,
            pdst
        );
        dst.set_len(dstlen);
        dst
    }
}
pub fn streamvbyte_decode(in_: &[u8], num_ints: usize) -> Vec<u32> {
    unsafe {
        let pin = in_.as_ptr();

        let mut dst = Vec::with_capacity(num_ints);
        let pdst = dst.as_mut_ptr();

        streamvbyte_sys::streamvbyte_decode(
            pin,
            pdst,
            num_ints as u32
        );
        dst.set_len(num_ints);
        dst
    }
}

#[test]
fn simple_encode_decode() {
    let data: Vec<u32> = vec![0,1,2,3,4,5,6,7,8,9,10];
    let encoded = streamvbyte_encode(&data);
    let decoded = streamvbyte_decode(&encoded, data.len());
    assert!(encoded.len() < data.len() * 4);
    assert_eq!(data, decoded);
}

pub fn streamvbyte_encode_0124(in_: &[u32]) -> Vec<u8> {
    unsafe {
        let len = in_.len();
        let pin = in_.as_ptr();

        let maxlen = streamvbyte_sys::streamvbyte_max_compressedbytes(len);
        let mut dst = Vec::with_capacity(maxlen);
        let pdst = dst.as_mut_ptr();

        let dstlen = streamvbyte_sys::streamvbyte_encode_0124(
            pin,
            len as u32,
            pdst
        );
        dst.set_len(dstlen);
        dst
    }
}
pub fn streamvbyte_decode_0124(in_: &[u8], num_ints: usize) -> Vec<u32> {
    unsafe {
        let pin = in_.as_ptr();

        let mut dst = Vec::with_capacity(num_ints);
        let pdst = dst.as_mut_ptr();

        streamvbyte_sys::streamvbyte_decode_0124(
            pin,
            pdst,
            num_ints as u32
        );
        dst.set_len(num_ints);
        dst
    }
}

#[test]
fn simple_encode_decode_0124() {
    let data: Vec<u32> = vec![0,1,2,3,4,5,6,7,8,9,10];
    let encoded = streamvbyte_encode_0124(&data);
    let decoded = streamvbyte_decode_0124(&encoded, data.len());
    assert!(encoded.len() < data.len() * 4);
    assert_eq!(data, decoded);
}

pub fn streamvbyte_delta_encode(in_: &[u32], prev: u32) -> Vec<u8> {
    unsafe {
        let len = in_.len();
        let pin = in_.as_ptr();

        let maxlen = streamvbyte_sys::streamvbyte_max_compressedbytes(len);
        let mut dst = Vec::with_capacity(maxlen);
        let pdst = dst.as_mut_ptr();

        let dstlen = streamvbyte_sys::streamvbyte_delta_encode(
            pin,
            len as u32,
            pdst,
            prev
        );
        dst.set_len(dstlen);
        dst
    }
}
pub fn streamvbyte_delta_decode(in_: &[u8], num_ints: usize, prev: u32) -> Vec<u32> {
    unsafe {
        let pin = in_.as_ptr();

        let mut dst = Vec::with_capacity(num_ints);
        let pdst = dst.as_mut_ptr();

        streamvbyte_sys::streamvbyte_delta_decode(
            pin,
            pdst,
            num_ints as u32,
            prev
        );
        dst.set_len(num_ints);
        dst
    }
}

#[test]
fn simple_encode_decode_delta() {
    let data: Vec<u32> = vec![0,1,2,3,4,5,6,7,8,9,10];
    let encoded = streamvbyte_delta_encode(&data, 0);
    let decoded = streamvbyte_delta_decode(&encoded, data.len(), 0);
    assert!(encoded.len() < data.len() * 4);
    assert_eq!(data, decoded);
}