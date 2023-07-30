use std::collections::HashMap;
#[cfg(feature = "bevy")]
use bevy::prelude::Resource;
use crate::pos::{ChunkPos2D, BlocPos, BlocPos2D};
use crate::Col;
pub type Cols<E> = HashMap<ChunkPos2D, E>;


#[cfg_attr(feature = "bevy", derive(Resource))]
pub struct Blocs<B: Default>(Cols<Col<B>>);

impl<B: Default + PartialEq + Clone + Copy> Blocs<B> {
    pub fn get_block(&self, pos: BlocPos) -> B {
        let (colpos, coledpos) = pos.into();
        match self.0.get(&colpos) {
            None => B::default(),
            Some(col) => col.get(coledpos)
        }
    }

    pub fn top_block(&self, pos: BlocPos2D) -> (B, i32) {
        let (colpos, pos2d) = pos.into();
        match self.0.get(&colpos) {
            None => (B::default(), 0),
            Some(col) => col.top(pos2d)
        }
    }
}