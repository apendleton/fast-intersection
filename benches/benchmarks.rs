#[macro_use]
extern crate criterion;
extern crate rand;
extern crate fast_intersection;
extern crate slow_intersection;

use criterion::Criterion;

mod intersections;

criterion_group!{
    name = benches;
    config = Criterion::default();

    targets = intersections::benchmark
}
criterion_main!(benches);
