use criterion::{black_box, criterion_group, criterion_main, Criterion};
use chunky::utils::{PackedUsizes, GetSet};
use rand::{Rng, seq::SliceRandom};
const N: usize = 1_000_000;

fn rand_read(packed: &PackedUsizes, indices: &Vec<usize>, out: &mut Vec<usize>) {
    for i in indices.into_iter() {
        out[*i] = packed.get(*i);
    }
}

fn rand_write(packed: &mut PackedUsizes, indices: &Vec<usize>, input: &Vec<usize>) {
    for i in indices.into_iter() {
        packed.set(*i, input[*i]);
    }
}

fn read_benchmark<const BITSIZE: u32>(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    // generate a random packed array
    let mut values = vec![0; N];
    values.fill_with(|| rng.gen_range(0..2_usize.pow(BITSIZE)));
    let packed = PackedUsizes::from_usizes(values, BITSIZE);
    // generate random indices
    let mut indices: Vec<usize> = (0..N).collect();
    indices.shuffle(&mut rng);
    // array for reading into
    let mut values = vec![0; N];
    c.bench_function(&format!("rand_read-1m-{}", BITSIZE), |b| b.iter(|| rand_read(
        black_box(&packed), black_box(&indices), black_box(&mut values)
    )));
}

fn write_benchmark<const BITSIZE: u32>(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    // generate a random array of values
    let mut values = vec![0; N];
    values.fill_with(|| rng.gen_range(0..2_usize.pow(BITSIZE)));
    // generate random indices
    let mut indices: Vec<usize> = (0..N).collect();
    indices.shuffle(&mut rng);
    // packed array to write to
    let mut packed = PackedUsizes::filled(N, BITSIZE, 0);
    c.bench_function(&format!("rand_write-1m-{}", BITSIZE), |b| b.iter(|| rand_write(
        black_box(&mut packed), black_box(&indices), black_box(&values)
    )));
}

criterion_group!(benches, read_benchmark<4>, write_benchmark<4>);
criterion_main!(benches);