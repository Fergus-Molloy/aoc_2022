use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc_2022::{solutions::{D1, Solution}, Input};

    fn pt_1(inp: Input) -> u64 {
        let calc_fuel = |x: u64| {
            let mut fuel:i64 = (x/3-2).try_into().unwrap_or(0);
            let mut acc= fuel;
            while fuel>0 {
                fuel = fuel/3-2;
                acc += if fuel > 0 {fuel} else {0}
            }
            acc.try_into().unwrap()
        };
        inp.lines().map(|s| s.parse().map(calc_fuel)).into_iter().collect::<Result<Vec<u64>,_>>().unwrap().iter().sum()
    }
    fn pt_2(inp: Input) -> u64 {
        let calc_fuel = |x: u64| {
            let mut fuel = (x/3-2) as i64;
            let mut acc= fuel;
            while fuel>0 {
                fuel = fuel/3-2;
                acc += if fuel > 0 {fuel} else {0}
            }
            acc as u64
        };
        inp.lines().map(|s| s.parse().map(calc_fuel)).into_iter().collect::<Result<Vec<u64>,_>>().unwrap().iter().sum()
    }
pub fn criterion_benchmark(c: &mut Criterion) {
    let inp = Input::load_from_day(1);
    c.bench_function("fn", |b| b.iter(|| pt_1(black_box(inp.clone()))));
    c.bench_function("closure", |b| b.iter(|| pt_2(black_box(inp.clone()))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
