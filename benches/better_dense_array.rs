use chunky::{utils::BetterDenseArray, CHUNK_S3};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{seq::SliceRandom, Rng};

fn rand_read(packed: &BetterDenseArray, indices: &Vec<usize>) {
    for i in indices.into_iter() {
        let _ = packed.get(*i);
    }
}

fn rand_write(packed: &mut BetterDenseArray, indices: &Vec<usize>) {
    for i in indices.into_iter() {
        packed.set(*i, 0);
    }
}

fn read_benchmark<const BITSIZE: u32>(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    // generate a random packed array
    let mut values = vec![0; CHUNK_S3];
    values.fill_with(|| rng.gen_range(0..2_usize.pow(BITSIZE)));
    let packed = BetterDenseArray::from(values.as_slice());
    // generate random indices
    let mut indices: Vec<usize> = (0..CHUNK_S3).collect();
    indices.shuffle(&mut rng);
    // array for reading into
    c.bench_function(&format!("bda_rand_read-32^3-{}", BITSIZE), |b| {
        b.iter(|| rand_read(black_box(&packed), black_box(&indices)))
    });
}

fn write_benchmark<const BITSIZE: u32>(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    // generate a random array of values
    let mut values = vec![0; CHUNK_S3];
    values.fill_with(|| rng.gen_range(0..2_usize.pow(BITSIZE)));
    // generate random indices
    let mut indices: Vec<usize> = (0..CHUNK_S3).collect();
    indices.shuffle(&mut rng);
    // packed array to write to
    let mut packed = BetterDenseArray::new();
    c.bench_function(&format!("bda_rand_write-32^3-{}", BITSIZE), |b| {
        b.iter(|| rand_write(black_box(&mut packed), black_box(&indices)))
    });
}

criterion_group!(dense_array, read_benchmark<4>, write_benchmark<4>);
criterion_main!(dense_array);
