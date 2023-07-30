use criterion::{black_box, criterion_group, criterion_main, Criterion};
use chunky::CHUNK_S3;
use rand::{Rng, seq::SliceRandom};

fn rand_read(packed: &Vec<usize>, indices: &Vec<usize>, out: &mut Vec<usize>) {
    for i in indices.into_iter() {
        out[*i] = packed[*i];
    }
}

fn rand_write(packed: &mut Vec<usize>, indices: &Vec<usize>, input: &Vec<usize>) {
    for i in indices.into_iter() {
        packed[*i] = input[*i];
    }
}

fn read_benchmark<const BITSIZE: u32>(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    // generate a random packed array
    let mut packed = vec![0; CHUNK_S3];
    packed.fill_with(|| rng.gen_range(0..2_usize.pow(BITSIZE)));
    let packed = packed.clone();
    // generate random indices
    let mut indices: Vec<usize> = (0..CHUNK_S3).collect();
    indices.shuffle(&mut rng);
    // array for reading into
    let mut values = vec![0; CHUNK_S3];
    c.bench_function(&format!("cv_rand_read-32^3-{}", BITSIZE), |b| b.iter(|| rand_read(
        black_box(&packed), black_box(&indices), black_box(&mut values)
    )));
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
    let mut packed = vec![0; CHUNK_S3];
    c.bench_function(&format!("cv_rand_write-32^3-{}", BITSIZE), |b| b.iter(|| rand_write(
        black_box(&mut packed), black_box(&indices), black_box(&values)
    )));
}

criterion_group!(control_vec, read_benchmark<4>, write_benchmark<4>);
criterion_main!(control_vec);