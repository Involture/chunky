use std::num::NonZeroU8;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use chunky::CHUNK_S3;
use rand::{Rng, seq::SliceRandom};
use unthbuf::{UnthBuf, aligned::AlignedLayout};

fn rand_read(packed: &UnthBuf<AlignedLayout>, indices: &Vec<usize>, out: &mut Vec<usize>) {
    for i in indices.into_iter() {
        out[*i] = packed.get(*i).unwrap();
    }
}

fn rand_write(packed: &mut UnthBuf<AlignedLayout>, indices: &Vec<usize>, input: &Vec<usize>) {
    for i in indices.into_iter() {
        packed.set(*i, input[*i]).unwrap();
    }
}

fn read_benchmark<const BITSIZE: u32>(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    // generate a random packed array
    let mut values = vec![0; CHUNK_S3];
    values.fill_with(|| rng.gen_range(0..2_usize.pow(BITSIZE)));
    let packed = UnthBuf::new_from_capacity_and_iter(
        NonZeroU8::try_from(BITSIZE as u8).unwrap(), CHUNK_S3, values.into_iter()
    );
    // generate random indices
    let mut indices: Vec<usize> = (0..CHUNK_S3).collect();
    indices.shuffle(&mut rng);
    // array for reading into
    let mut values = vec![0; CHUNK_S3];
    c.bench_function(&format!("ub_rand_read-32^3-{}", BITSIZE), |b| b.iter(|| rand_read(
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
    let mut packed = UnthBuf::new_with_default(
        NonZeroU8::try_from(BITSIZE as u8).unwrap(), CHUNK_S3, 0
    );
    c.bench_function(&format!("ub_rand_write-32^3-{}", BITSIZE), |b| b.iter(|| rand_write(
        black_box(&mut packed), black_box(&indices), black_box(&values)
    )));
}

criterion_group!(unthbuf, read_benchmark<4>, write_benchmark<4>);
criterion_main!(unthbuf);