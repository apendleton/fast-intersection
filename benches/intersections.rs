use criterion::{Criterion, Fun, Bencher};
use fast_intersection::*;
use slow_intersection::*;
use std::rc::Rc;
use rand;
use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use std::collections::HashSet;

pub fn benchmark(c: &mut Criterion) {
    // the things I'm going to set up once and share across benchmarks are a list of words
    // and a built prefix set, so define a struct to contain them
    struct BenchData {
        small: Vec<Vec<u32>>,
        medium: Vec<Vec<u32>>,
        large: Vec<Vec<u32>>
    };

    let mut rng = rand::thread_rng();
    let mut bd = BenchData { small: Vec::new(), medium: Vec::new(), large: Vec::new() };
    for (count, group) in vec![(100, &mut bd.small), (1000, &mut bd.medium), (100000, &mut bd.large)] {
        let values = Uniform::from(0..(10 * count as u32));
        let mut set: HashSet<u32> = HashSet::new();

        while set.len() < count {
            set.insert(values.sample(&mut rng));
        }
        let mut set: Vec<u32> = set.into_iter().collect();
        set.sort();

        group.push(set);
    }

    // move the prebuilt data into a reference-counted struct
    let shared_data = Rc::new(bd);

    // make a vector I'm going to fill with closures to bench-test
    let mut to_bench = Vec::new();

    let funcs: Vec<(&str, &Fn(&[u32], &[u32]) -> Vec<u32>)> = vec![
        ("simd", &simd_intersection),
        ("simd_avx2", &simd_intersection_avx2),
        ("merge", &merge_intersection),
        ("hash", &hashset_intersection),
    ];
    for (name, intersection_func) in funcs {
        // not a real clone, just a refcount bump
        let data = shared_data.clone();
        to_bench.push(Fun::new(&(name.to_owned() + "_small_identical"), move |b: &mut Bencher, _i| {
            let mut cycle = data.small.iter().cycle();

            b.iter(|| {
                let v = cycle.next().unwrap();
                intersection_func(v, v);
            });
        }));

        let data = shared_data.clone();
        to_bench.push(Fun::new(&(name.to_owned() + "_medium_identical"), move |b: &mut Bencher, _i| {
            let mut cycle = data.medium.iter().cycle();

            b.iter(|| {
                let v = cycle.next().unwrap();
                intersection_func(v, v);
            });
        }));

        let data = shared_data.clone();
        to_bench.push(Fun::new(&(name.to_owned() + "_large_identical"), move |b: &mut Bencher, _i| {
            let mut cycle = data.medium.iter().cycle();

            b.iter(|| {
                let v = cycle.next().unwrap();
                intersection_func(v, v);
            });
        }));

        let data = shared_data.clone();
        to_bench.push(Fun::new(&(name.to_owned() + "_small_balanced"), move |b: &mut Bencher, _i| {
            let mut cycle = data.small.iter().cycle();
            let mut rng = rand::thread_rng();

            let mut other_vecs: Vec<&Vec<u32>> = Vec::new();
            for _i in 0..1000 {
                other_vecs.push(rng.choose(&data.small).unwrap());
            }
            let mut cycle2 = other_vecs.iter().cycle();

            b.iter(|| {
                let v1 = cycle.next().unwrap();
                let v2 = cycle2.next().unwrap();
                intersection_func(v1, *v2);
            });
        }));

        let data = shared_data.clone();
        to_bench.push(Fun::new(&(name.to_owned() + "_medium_balanced"), move |b: &mut Bencher, _i| {
            let mut cycle = data.medium.iter().cycle();
            let mut rng = rand::thread_rng();

            let mut other_vecs: Vec<&Vec<u32>> = Vec::new();
            for _i in 0..1000 {
                other_vecs.push(rng.choose(&data.medium).unwrap());
            }
            let mut cycle2 = other_vecs.iter().cycle();

            b.iter(|| {
                let v1 = cycle.next().unwrap();
                let v2 = cycle2.next().unwrap();
                intersection_func(v1, *v2);
            });
        }));

        let data = shared_data.clone();
        to_bench.push(Fun::new(&(name.to_owned() + "_large_balanced"), move |b: &mut Bencher, _i| {
            let mut cycle = data.large.iter().cycle();
            let mut rng = rand::thread_rng();

            let mut other_vecs: Vec<&Vec<u32>> = Vec::new();
            for _i in 0..1000 {
                other_vecs.push(rng.choose(&data.large).unwrap());
            }
            let mut cycle2 = other_vecs.iter().cycle();

            b.iter(|| {
                let v1 = cycle.next().unwrap();
                let v2 = cycle2.next().unwrap();
                intersection_func(v1, *v2);
            });
        }));

        let data = shared_data.clone();
        to_bench.push(Fun::new(&(name.to_owned() + "_small_large"), move |b: &mut Bencher, _i| {
            let mut cycle = data.small.iter().cycle();
            let mut rng = rand::thread_rng();

            let mut other_vecs: Vec<&Vec<u32>> = Vec::new();
            for _i in 0..1000 {
                other_vecs.push(rng.choose(&data.large).unwrap());
            }
            let mut cycle2 = other_vecs.iter().cycle();

            b.iter(|| {
                let v1 = cycle.next().unwrap();
                let v2 = cycle2.next().unwrap();
                intersection_func(v1, *v2);
            });
        }));

        let data = shared_data.clone();
        to_bench.push(Fun::new(&(name.to_owned() + "_large_small"), move |b: &mut Bencher, _i| {
            let mut cycle = data.large.iter().cycle();
            let mut rng = rand::thread_rng();

            let mut other_vecs: Vec<&Vec<u32>> = Vec::new();
            for _i in 0..1000 {
                other_vecs.push(rng.choose(&data.small).unwrap());
            }
            let mut cycle2 = other_vecs.iter().cycle();

            b.iter(|| {
                let v1 = cycle.next().unwrap();
                let v2 = cycle2.next().unwrap();
                intersection_func(v1, *v2);
            });
        }));
    }

    // run the accumulated list of benchmarks
    c.bench_functions("intersections", to_bench, ());
}