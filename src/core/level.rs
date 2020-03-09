use rand::Rng;
use crate::core::terrain::BlockType;

pub enum Level {
    EASY,
    MEDIUM,
    HARD
}

pub fn create(level: Level) -> Vec<Vec<BlockType>> {
    return match level {
        Level::EASY => create_level((10, 10),10),
        Level::MEDIUM => create_level((15, 15),20),
        Level::HARD => create_level((25, 25), 30),
        (_) => create_level((10, 10), 10)
    }
}

fn create_level(size: (usize, usize), ratio: u32) -> Vec<Vec<BlockType>> {
    let mut rng = rand::thread_rng();
    let mut rand_num: u32;
    let mut area: Vec<Vec<BlockType>> = Vec::new();
    for x in 0..size.0 {
        area.push(Vec::new());
        for _ in 0..size.1 {
            let r = rng.gen_range(0, 100);
            if r <= ratio {
                area[x].push(BlockType::BOMB);
            } else {
                area[x].push(BlockType::CLOSED);
            }
        }
    }
    return area
}