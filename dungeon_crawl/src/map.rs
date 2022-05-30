use crate::prelude::*;

// usize will use the preferred bit size for your CPU, 
// e.g. 64 bit computer will be 64 bits
const NUM_TILES: usize = (SCREEN_W * SCREEN_H) as usize;

// Clone -> .clone() method for deep copy without affecting the original
// Copy -> don't move the value take a copy when re-assigning to variable
// PartialEq -> Compare TileType with the == operator
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor
}

pub struct Map {
    pub tiles: Vec<TileType>
}


// Vectors are indexed on a single dimension so to convert an (x,y) point
// to vector indicies we use a transformation called "striding".
// Specifically, we use "row-first encoding":
// y
// | 1 | 2 | 3 |
// | 4 | 5 | 6 |
//   --- x --->
// This function computes the vector index from the (x,y) map position
pub fn map_index(x: i32, y: i32) -> usize {
    ((y * SCREEN_W) + x) as usize
}

impl Map {
    pub fn try_index(&self, p : Point) -> Option<usize> {
        if !self.in_bounds(p) {
            None
        } else {
            Some(map_index(p.x, p.y))
        }
    }

    pub fn can_enter_tile(&self, p : Point) -> bool {
        self.in_bounds(p) && self.tiles[map_index(p.x, p.y)] == TileType::Floor
    }

    pub fn new() -> Self {
        Self {
            // This is an extended macro creating NUM_TILES of items which are
            // of type TileType::Floor
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn in_bounds(&self, point : Point) -> bool{
        point.x >= 0 && point.y < SCREEN_W  && point.y >= 0 && point.y < SCREEN_H
    }

    pub fn render(&self, ctx: &mut BTerm) {
        for y in 0..SCREEN_H {
            for x in 0..SCREEN_W {
                let i = map_index(x,y);

                match self.tiles[i] {
                    TileType::Floor => {
                        ctx.set(x,y, YELLOW, BLACK, to_cp437(','));
                    }
                    TileType::Wall => {
                        ctx.set(x,y, GREEN, BLACK, to_cp437('#'));
                    }
                }
            }
        }
    }
}
