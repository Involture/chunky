use std::ops::IndexMut;
use crate::{blocs::{Chunk, CHUNK_S1}, pos::{ChunkedPos2D, ColedPos, bloc_pos::chunked}, utils::PackedUsizes};
use itertools::iproduct;
pub const MAX_HEIGHT: usize = 256;

pub struct Col<B: Default> {
    pub chunks: [Option<Chunk<B>>; MAX_HEIGHT / CHUNK_S1],
}

impl<B: Default + PartialEq + Clone + Copy> Col<B> {
    pub fn new() -> Self {
        Col {
            chunks: core::array::from_fn(|_| None),
        }
    }

    pub fn top(&self, (x, z): ChunkedPos2D) -> (B, i32) {
        for cy in (0..self.chunks.len()).rev() {
            if let Some(chunk) = &self.chunks[cy] {
                for dy in (0..CHUNK_S1).rev() {
                    let bloc = chunk.get((x, dy, z));
                    if *bloc != B::default() {
                        return (*bloc, (cy * CHUNK_S1 + dy) as i32);
                    }
                }
            }
        }
        (B::default(), 0)
    }

    pub fn get(&self, (dx, y, dz): ColedPos) -> B {
        let (qy, dy) = chunked(y);
        let qy = qy as usize;
        match &self.chunks[qy] {
            None => B::default(),
            Some(chunk) => chunk.get((dx, dy, dz)).clone()
        }
    }

    pub fn set(&mut self, (dx, y, dz): ColedPos, bloc: B) {
        let (qy, dy) = chunked(y);
        let qy = qy as usize;
        if self.chunks[qy].is_none() {
            self.chunks[qy] = Some(Chunk::<B, PackedUsizes>::new());
        }
        self.chunks[qy].as_mut().unwrap().set((dx, dy, dz), bloc);
    }

    pub fn fill_up(&mut self, bloc: B) {
        let mut qy = 0;
        // fill the uninitialized chunks
        while self.chunks[qy].is_none() {
            self.chunks[qy] = Some(Chunk::filled(bloc));
            qy += 1;
        }
        // fill the first initialized chunk until the first non-air block (if there's one) 
        for (dx, dz) in iproduct!(0..CHUNK_S1, 0..CHUNK_S1) {
            let mut dy: usize = 0;
            let chunk = self.chunks.index_mut(qy).as_mut().unwrap();
            while dy < CHUNK_S1 && chunk.set_if_empty((dx, dy, dz), bloc) {
                dy += 1;
            }
        }
    }
}