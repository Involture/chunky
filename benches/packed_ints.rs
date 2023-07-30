use criterion::{black_box, criterion_group, criterion_main, Criterion};
use chunky::{utils::{PackedUsizes, GetSet}, CHUNK_S3};
use rand::{Rng, seq::SliceRandom};
const N: usize = CHUNK_S3;

fn rand_read(packed: &PackedUsizes, indices: &Vec<usize>) {
    for i in indices.into_iter() {
        let _ = packed.get(*i);
    }
}

fn rand_write(packed: &mut PackedUsizes, indices: &Vec<usize>) {
    for i in indices.into_iter() {
        packed.set(*i, 0);
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
    c.bench_function(&format!("pi_rand_read-32^3-{}", BITSIZE), |b| b.iter(|| rand_read(
        black_box(&packed), black_box(&indices)
    )));
}

fn write_benchmark<const BITSIZE: u32>(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    // generate random indices
    let mut indices: Vec<usize> = (0..N).collect();
    indices.shuffle(&mut rng);
    // packed array to write to
    let mut packed = PackedUsizes::filled(N, BITSIZE, 0);
    c.bench_function(&format!("pi_rand_write-32^3-{}", BITSIZE), |b| b.iter(|| rand_write(
        black_box(&mut packed), black_box(&indices)
    )));
}

criterion_group!(packed_ints, read_benchmark<4>, write_benchmark<4>);
criterion_main!(packed_ints);