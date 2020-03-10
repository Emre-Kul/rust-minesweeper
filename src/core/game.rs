use std::io;
use std::io::stdin;
use std::ops::Add;
use rand::Rng;

use crate::core::terrain::{Terrain, BlockType};
use crate::core::level::Level;
use std::borrow::Borrow;

pub struct Game {
    pub is_bomb_found: bool,
    pub terrain: Terrain,
}

impl Game {

    pub fn create() -> Game {
        Game {
            is_bomb_found: false,
            terrain: Terrain::create(Level::EASY),
        }
    }

    pub fn open_block(&mut self, coords: (usize, usize)) {
        if self.terrain.blocks[coords.0][coords.1] == BlockType::BOMB {
            self.is_bomb_found = true;
        } else {
            let count = self.count_neigh_bomb((coords.0 as isize, coords.1 as isize));
            let neighs = self.get_closed_neighs((coords.0 as isize, coords.1 as isize));
            self.terrain.blocks[coords.0][coords.1] = BlockType::OPEN(count as u32);
            if(count == 0) {
                for n in neighs.iter() {
                    self.open_block((n.0 as usize, n.1 as usize))
                }
            }

        }
    }

    pub fn is_finished(&self) -> bool {
        self.is_bomb_found == false && self.has_closed_block()
    }

    fn has_closed_block(&self) -> bool {
        for i in self.terrain.blocks.iter() {
            for block in i.iter() {
                if block.eq(&BlockType::CLOSED) {
                    return true;
                }
            }
        }
        return false;
    }

    fn count_neigh_bomb(&self, coords: (isize, isize)) -> u32 {
        let mut count: u32 = 0;
        let size: isize = self.terrain.blocks.len() as isize;
        for i in (coords.0 - 1)..(coords.0 + 2) {
            for j in (coords.1 - 1)..(coords.1 + 2) {
                if (i == coords.0 && j == coords.1) || i < 0 || j < 0 || i >= size || j >= size {
                    continue;
                }
                if(self.terrain.blocks[i as usize][j as usize] == BlockType::BOMB) {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_closed_neighs(&self, coords: (isize, isize)) -> Vec<(isize, isize)> {
        let mut vec: Vec<(isize, isize)> = Vec::new();
        let size: isize = self.terrain.blocks.len() as isize;
        for i in (coords.0 - 1)..(coords.0 + 2) {
            for j in (coords.1 - 1)..(coords.1 + 2) {
                if (i == coords.0 && j == coords.1) || i < 0 || j < 0 || i >= size || j >= size {
                    continue;
                }
                if(self.terrain.blocks[i as usize][j as usize] == BlockType::CLOSED) {
                    vec.push((i, j));
                }
            }
        }
        vec
    }
}