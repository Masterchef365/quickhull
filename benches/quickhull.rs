use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quickhull::*;
use rand::distributions::{Distribution, Uniform};
use rand::rngs::SmallRng;
use rand::SeedableRng;

fn uniform_box(n: u64) -> Vec<Point> {
    let mut rng = SmallRng::seed_from_u64(n);
    let unif = Uniform::new(-1., 1.);
    let mut points = Vec::with_capacity(n as _);
    for _ in 0..n {
        let x = unif.sample(&mut rng);
        let y = unif.sample(&mut rng);
        points.push(Point::new(x, y));
    }
    points
}

fn uniform_box_test(n: u64, c: &mut Criterion) {
    let data = uniform_box(n);
    c.bench_function(&format!("uniform_box {}", n), |b| {
        b.iter(|| quickhull(black_box(&data)))
    });
}

fn uniform_box_test_200_000(c: &mut Criterion) {
    uniform_box_test(200_000, c)
}

fn uniform_box_test_2000(c: &mut Criterion) {
    uniform_box_test(2000, c)
}

fn uniform_box_test_100(c: &mut Criterion) {
    uniform_box_test(100, c)
}

criterion_group!(
    benches,
    uniform_box_test_100,
    uniform_box_test_2000,
    uniform_box_test_200_000
);
criterion_main!(benches);