use crate::core::level;

#[derive(PartialEq, Eq)]
pub enum BlockType {
    CLOSED,
    BOMB,
    OPEN(u32)
}

pub struct Terrain {
    pub blocks: Vec<Vec<BlockType>>
}


impl Terrain {
    pub fn create(level: level::Level) -> Terrain {
        Terrain {
            blocks: level::create(level)
        }
    }
}
