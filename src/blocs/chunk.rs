use crate::pos::ChunkedPos;
use crate::utils::{
    find_bitsize, GetSet, PackedUsizes, Palette
};
pub const CHUNK_S1: usize = 32;
pub const CHUNK_S2: usize = CHUNK_S1.pow(2);
pub const CHUNK_S3: usize = CHUNK_S1.pow(3);

fn index(x: usize, y: usize, z: usize) -> usize {
    x + y * CHUNK_S1 + z * CHUNK_S2
}

#[derive(Debug)]
pub struct Chunk<B: Default, L: GetSet<usize> = PackedUsizes> {
    data: L,
    palette: Vec<B>,
}

impl<B: Default + PartialEq, L: GetSet<usize>> Chunk<B, L> {
    pub fn get(&self, (x, y, z): ChunkedPos) -> &B {
        &self.palette[self.data.get(index(x, y, z))]
    }

    pub fn set(&mut self, (x, y, z): ChunkedPos, bloc: B) {
        let idx = index(x, y, z);
        self.data.set(idx, self.palette.index(bloc));
    }

    pub fn set_if_empty(&mut self, (x, y, z): ChunkedPos, bloc: B) -> bool {
        let idx = index(x, y, z);
        if self.palette[self.data.get(idx)] != B::default() {
            return false;
        }
        self.data.set(idx, self.palette.index(bloc));
        true
    }
}

impl<B: Default + PartialEq> Chunk<B, PackedUsizes> {
    pub fn new() -> Self {
        Chunk {
            data: PackedUsizes::new(CHUNK_S3, 4),
            palette: vec![B::default()],
        }
    }

    pub fn filled(bloc: B) -> Self {
        if bloc == B::default() {
            Chunk::<B, PackedUsizes>::new()
        } else {
            Chunk {
                data: PackedUsizes::filled(CHUNK_S3, 4, 1),
                palette: vec![B::default(), bloc],
            }
        }
    }
}

impl<B: Default> Chunk<B, Vec<usize>> {
    pub fn new() -> Self {
        Chunk {
            data: vec![0; CHUNK_S3],
            palette: vec![B::default()],
        }
    }
}

impl<B: Default> From<Chunk<B, Vec<usize>>> for Chunk<B, PackedUsizes> {
    fn from(chunk: Chunk<B, Vec<usize>>) -> Self {
        Chunk {
            data: PackedUsizes::from_usizes(chunk.data, find_bitsize(chunk.palette.len())),
            palette: chunk.palette,
        }
    }
}

impl<B: Default> Default for Chunk<B, Vec<usize>> {
    fn default() -> Self {
        Self::new()
    }
}

impl<B: Default + PartialEq> Default for Chunk<B, PackedUsizes> {
    fn default() -> Self {
        Self::new()
    }
}
