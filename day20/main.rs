#![feature(destructuring_assignment)]
use std::collections::HashMap;
use std::{fs::File, io, io::prelude::*};
use unicode_segmentation::UnicodeSegmentation;

const WIDTH: usize = 10;
const HEIGHT: usize = 10;
const PLACEMENT_SIDE: usize = 12;

#[derive(Debug, PartialEq, Clone)]
enum Symmetry {
    I,
    J,
    R,
    R2,
    R3,
    JR,
    JR2,
    JR3,
}

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    // 0 = top, 1 = right, 2 = bottom, 3 = left
    borders: [String; 4],
    symmetry: Symmetry,
}

impl Tile {
    fn new(id: usize, top: String, right: String, bottom: String, left: String) -> Tile {
        Tile {
            id: id,
            borders: [top, right, bottom, left],
            symmetry: Symmetry::I,
        }
    }

    fn act(&mut self) {
        match self.symmetry {
            Symmetry::I => {
                self.rotate_borders();
                self.symmetry = Symmetry::R;
            }
            Symmetry::R => {
                self.rotate_borders();
                self.symmetry = Symmetry::R2;
            }
            Symmetry::R2 => {
                self.rotate_borders();
                self.symmetry = Symmetry::R3;
            }
            Symmetry::R3 => {
                self.rotate_borders();
                self.rotate_borders();
                self.rotate_borders();
                self.reflect_borders();
                self.symmetry = Symmetry::JR2;
            }
            Symmetry::JR2 => {
                self.reflect_borders();
                self.rotate_borders();
                self.reflect_borders();
                self.symmetry = Symmetry::JR3;
            }
            Symmetry::JR3 => {
                self.reflect_borders();
                self.rotate_borders();
                self.reflect_borders();
                self.symmetry = Symmetry::J;
            }
            Symmetry::J => {
                self.reflect_borders();
                self.rotate_borders();
                self.reflect_borders();
                self.symmetry = Symmetry::JR;
            }
            Symmetry::JR => {
                self.reflect_borders();
                self.rotate_borders();
                self.rotate_borders();
                self.rotate_borders();
                self.symmetry = Symmetry::I;
            }
        }
    }

    fn reverse(word: String) -> String {
        word.graphemes(true).rev().collect()
    }

    fn rotate_borders(&mut self) {
        let (top, right, bottom, left) = (
            self.borders[0].clone(),
            self.borders[1].clone(),
            self.borders[2].clone(),
            self.borders[3].clone(),
        );

        self.borders[0] = Tile::reverse(left.to_string());
        self.borders[1] = top.to_string();
        self.borders[2] = Tile::reverse(right.to_string());
        self.borders[3] = bottom.to_string();
    }

    fn reflect_borders(&mut self) {
        let (top, right, bottom, left) = (
            self.borders[0].clone(),
            self.borders[1].clone(),
            self.borders[2].clone(),
            self.borders[3].clone(),
        );

        // top becomes the bottom and vice versa
        self.borders[2] = top;
        self.borders[0] = bottom;

        // sides are reflected
        self.borders[1] = Tile::reverse(right.to_string());
        self.borders[3] = Tile::reverse(left.to_string());
    }

    fn to_identity(&mut self) {
        match self.symmetry {
            Symmetry::I => {}
            Symmetry::J => {
                self.reflect_borders();
            }
            Symmetry::JR => {
                self.reflect_borders();
                self.rotate_borders();
                self.rotate_borders();
                self.rotate_borders();
            }
            Symmetry::JR2 => {
                self.reflect_borders();
                self.rotate_borders();
                self.rotate_borders();
            }
            Symmetry::JR3 => {
                self.reflect_borders();
                self.rotate_borders();
            }
            Symmetry::R3 => {
                self.rotate_borders();
            }
            Symmetry::R => {
                self.rotate_borders();
                self.rotate_borders();
                self.rotate_borders();
            }
            Symmetry::R2 => {
                self.rotate_borders();
                self.rotate_borders();
            }
        }
        self.symmetry = Symmetry::I;
    }
}

struct Grid {
    placement: [[usize; PLACEMENT_SIDE]; PLACEMENT_SIDE],
    next_empty: (usize, usize),

    // always in identity orientation
    unplaced_tiles: HashMap<usize, Tile>,
    // always in factual orientation
    placed_tiles: HashMap<usize, Tile>,
}

impl Grid {
    fn new(tiles: HashMap<usize, Tile>) -> Grid {
        Grid {
            placement: [[0; PLACEMENT_SIDE]; PLACEMENT_SIDE],
            placed_tiles: HashMap::new(),
            next_empty: (0, 0),
            unplaced_tiles: tiles,
        }
    }

    fn advance(&mut self, tile: Tile) -> bool {
        let id = tile.id;
        self.placed_tiles.insert(tile.id, tile);
        self.placement[self.next_empty.0][self.next_empty.1] = id;
        self.next_empty = Grid::next(self.next_empty.0, self.next_empty.1);
        self.unplaced_tiles.remove(&id);

        if self.next_empty.0 == PLACEMENT_SIDE && self.next_empty.1 == 0 {
            return true;
        }

        false
    }

    fn backtrack(&mut self) {
        if self.next_empty.0 == 0 && self.next_empty.1 == 0 {
            return;
        }

        let last_pos = Grid::last(self.next_empty.0, self.next_empty.1);
        let last_id = self.placement[last_pos.0][last_pos.1];
        if let Some(last_tile) = self.placed_tiles.get_mut(&last_id) {
            // insert the last tile into the unplaced tiles
            last_tile.to_identity();
            let new_tile = last_tile.clone();
            self.unplaced_tiles.insert(last_id, new_tile);
        }

        // remove the last tile from placed tiles
        self.placed_tiles.remove(&last_id);
        self.placement[last_pos.0][last_pos.1] = 0;

        self.next_empty = last_pos;
    }

    fn next(i: usize, j: usize) -> (usize, usize) {
        let mut next_j = j + 1;
        let next_i = if next_j == PLACEMENT_SIDE {
            next_j = 0;
            i + 1
        } else {
            i
        };

        (next_i, next_j)
    }

    fn last(i: usize, j: usize) -> (usize, usize) {
        let last_i = if j == 0 { i - 1 } else { i };
        let last_j = if j == 0 { PLACEMENT_SIDE - 1 } else { j - 1 };

        (last_i, last_j)
    }

    fn get_constraint_borders(&mut self) -> (Option<String>, Option<String>) {
        let top_border = if self.next_empty.0 == 0 {
            None
        } else {
            let top_tile_id = self.placement[self.next_empty.0 - 1][self.next_empty.1];
            let top_tile = self.placed_tiles.get(&top_tile_id).unwrap();

            Some(top_tile.borders[2].clone())
        };

        let left_border = if self.next_empty.1 == 0 {
            None
        } else {
            let left_tile_id = self.placement[self.next_empty.0][self.next_empty.1 - 1];
            let left_tile = self.placed_tiles.get(&left_tile_id).unwrap();

            Some(left_tile.borders[1].clone())
        };

        (top_border, left_border)
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day20/input.txt")?.read_to_string(&mut buffer)?;
    let input: Vec<&str> = buffer.split("\n\n").collect();
    let mut grid = parse_input(input);
    println!("{}", backtrack(&mut grid));
    let id0 = grid.placement[0][0];
    let id1 = grid.placement[0][PLACEMENT_SIDE - 1];
    let id2 = grid.placement[PLACEMENT_SIDE - 1][0];
    let id3 = grid.placement[PLACEMENT_SIDE - 1][PLACEMENT_SIDE - 1];

    println!("{}", id0 * id1 * id2 * id3);

    Ok(())
}

fn backtrack(grid: &mut Grid) -> bool {
    let unplaced_tiles = grid.unplaced_tiles.clone();
    for tile in unplaced_tiles.values() {
        let mut new_tile = tile.clone();
        loop {
            let (top_border, left_border) = grid.get_constraint_borders();
            let top_border_ok = if let Some(top_border) = top_border {
                new_tile.borders[0] == top_border
            } else {
                true
            };

            let left_border_ok = if let Some(left_border) = left_border {
                new_tile.borders[3] == left_border
            } else {
                true
            };

            if top_border_ok && left_border_ok {
                if grid.advance(new_tile.clone()) {
                    println!("{:?}", grid.placement);
                    return true;
                }

                if backtrack(grid) {
                    return true;
                } else {
                    grid.backtrack();
                }
            }

            new_tile.act();
            if new_tile.symmetry == Symmetry::I {
                break;
            }
        }
    }

    return false;
}

fn parse_input(input: Vec<&str>) -> Grid {
    let mut tiles: HashMap<usize, Tile> = HashMap::new();

    for tile_proto in input {
        let mut lines = tile_proto.split("\n");
        let mut header = lines.next().unwrap().to_string();
        header.retain(|c| c.is_numeric());
        let id = header.parse::<usize>().unwrap();
        let tile_lines = lines.collect::<Vec<&str>>();

        let mut left = String::from("");
        let mut right = String::from("");
        for i in 0..tile_lines.len() {
            right.push(tile_lines[i].chars().nth(WIDTH - 1).unwrap());
            left.push(tile_lines[i].chars().next().unwrap());
        }

        let top = tile_lines[0].to_string();
        let bottom = tile_lines[HEIGHT - 1].to_string();
        let tile = Tile::new(id, top, right, bottom, left);
        tiles.insert(id, tile);
    }

    Grid::new(tiles)
}
