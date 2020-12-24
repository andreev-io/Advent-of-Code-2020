#![feature(destructuring_assignment)]
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::{fs::File, io, io::prelude::*};

// THIS NEEDS TO BE UPDATED BASED ON INPUT (sqrt(tiles))
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
    rows: Vec<String>,
    symmetry: Symmetry,
}

enum Border {
    Top,
    Right,
    Bottom,
    Left,
}

impl Tile {
    fn new(id: usize, rows: Vec<String>) -> Tile {
        Tile {
            id: id,
            rows: rows,
            symmetry: Symmetry::I,
        }
    }

    fn act(&mut self) {
        match self.symmetry {
            Symmetry::I => {
                self.rotate();
                self.symmetry = Symmetry::R;
            }
            Symmetry::R => {
                self.rotate();
                self.symmetry = Symmetry::R2;
            }
            Symmetry::R2 => {
                self.rotate();
                self.symmetry = Symmetry::R3;
            }
            Symmetry::R3 => {
                self.rotate();
                self.rotate();
                self.rotate();
                self.reflect();
                self.symmetry = Symmetry::JR2;
            }
            Symmetry::JR2 => {
                self.reflect();
                self.rotate();
                self.reflect();
                self.symmetry = Symmetry::JR3;
            }
            Symmetry::JR3 => {
                self.reflect();
                self.rotate();
                self.reflect();
                self.symmetry = Symmetry::J;
            }
            Symmetry::J => {
                self.reflect();
                self.rotate();
                self.reflect();
                self.symmetry = Symmetry::JR;
            }
            Symmetry::JR => {
                self.reflect();
                self.rotate();
                self.rotate();
                self.rotate();
                self.symmetry = Symmetry::I;
            }
        }
    }

    fn get_border(&self, border: Border) -> String {
        match border {
            Border::Top => self.rows[0].clone(),
            Border::Right => {
                let mut res = String::from("");
                for i in 0..self.rows.len() {
                    res.push(self.rows[i].chars().nth(9).unwrap());
                }
                res
            }
            Border::Bottom => self.rows[9].clone(),
            Border::Left => {
                let mut res = String::from("");
                for i in 0..self.rows.len() {
                    res.push(self.rows[i].chars().nth(0).unwrap());
                }
                res
            }
        }
    }

    fn rotate(&mut self) {
        let mut source_chars: Vec<Vec<char>> = Vec::new();

        for i in 0..self.rows.len() {
            source_chars.push(self.rows[i].chars().collect());
        }

        for i in 0..self.rows.len() / 2 {
            for j in 0..(self.rows.len() + 1) / 2 {
                let temp = source_chars[i][j];
                source_chars[i][j] = source_chars[self.rows.len() - 1 - j][i];
                source_chars[self.rows.len() - 1 - j][i] =
                    source_chars[self.rows.len() - 1 - i][self.rows.len() - 1 - j];
                source_chars[self.rows.len() - 1 - i][self.rows.len() - 1 - j] =
                    source_chars[j][self.rows.len() - 1 - i];
                source_chars[j][self.rows.len() - 1 - i] = temp;
            }
        }

        for (i, row) in source_chars.iter().enumerate() {
            self.rows[i] = row.iter().collect();
        }
    }

    fn reflect(&mut self) {
        let mut new_rows: Vec<String> = Vec::new();
        for _ in 0..self.rows.len() {
            new_rows.push(String::from(""));
        }

        for i in 0..self.rows.len() {
            new_rows[i] = self.rows[self.rows.len() - i - 1].clone();
            new_rows[self.rows.len() - i - 1] = self.rows[i].clone();
        }

        self.rows = new_rows;
    }

    fn to_identity(&mut self) {
        match self.symmetry {
            Symmetry::I => {}
            Symmetry::J => {
                self.reflect();
            }
            Symmetry::JR => {
                self.reflect();
                self.rotate();
                self.rotate();
                self.rotate();
            }
            Symmetry::JR2 => {
                self.reflect();
                self.rotate();
                self.rotate();
            }
            Symmetry::JR3 => {
                self.reflect();
                self.rotate();
            }
            Symmetry::R3 => {
                self.rotate();
            }
            Symmetry::R => {
                self.rotate();
                self.rotate();
                self.rotate();
            }
            Symmetry::R2 => {
                self.rotate();
                self.rotate();
            }
        }
        self.symmetry = Symmetry::I;
    }

    fn drop_borders(&mut self) {
        self.rows.remove(9);
        self.rows.remove(0);

        let mut new_rows: Vec<String> = Vec::new();
        for row in self.rows.iter() {
            let mut chars: Vec<char> = row.clone().chars().collect();
            chars.remove(9);
            chars.remove(0);
            new_rows.push(chars.iter().collect());
        }

        self.rows = new_rows;
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

            Some(top_tile.get_border(Border::Bottom))
        };

        let left_border = if self.next_empty.1 == 0 {
            None
        } else {
            let left_tile_id = self.placement[self.next_empty.0][self.next_empty.1 - 1];
            let left_tile = self.placed_tiles.get(&left_tile_id).unwrap();

            Some(left_tile.get_border(Border::Right))
        };

        (top_border, left_border)
    }

    fn drop_borders(&mut self) {
        for tile in self.placed_tiles.values_mut() {
            tile.drop_borders();
        }
    }

    fn into_tile(&self) -> Tile {
        let mut rows: Vec<String> = Vec::new();
        for row in self.placement.iter() {
            for (i, tile_id) in row.iter().enumerate() {
                let tile = self.placed_tiles.get(tile_id).unwrap();

                // if this is the first tile in the placement row, add its rows to the result rows
                if i == 0 {
                    for tile_row in &tile.rows {
                        rows.push(tile_row.to_string());
                    }
                // if it isn't the first, append its rows to existing rows
                } else {
                    let num_tile_rows = tile.rows.len();
                    for (i, tile_row) in tile.rows.iter().enumerate() {
                        let len = rows.len();
                        rows[len - num_tile_rows + i].push_str(tile_row);
                    }
                }
            }
        }

        Tile::new(0, rows)
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day20/input.txt")?.read_to_string(&mut buffer)?;
    let input: Vec<&str> = buffer.split("\n\n").collect();
    let mut grid = parse_input(input);
    backtrack(&mut grid);
    let id0 = grid.placement[0][0];
    let id1 = grid.placement[0][PLACEMENT_SIDE - 1];
    let id2 = grid.placement[PLACEMENT_SIDE - 1][0];
    let id3 = grid.placement[PLACEMENT_SIDE - 1][PLACEMENT_SIDE - 1];

    println!(
        "Answer 1: {} <- If you're getting 0, double check the PLACEMENT_SIDE constant",
        id0 * id1 * id2 * id3
    );

    grid.drop_borders();
    let t = grid.into_tile();

    println!("Answer 2: {}", check_for_sea_monsters(t));
    Ok(())
}

fn backtrack(grid: &mut Grid) -> bool {
    let unplaced_tiles = grid.unplaced_tiles.clone();
    for tile in unplaced_tiles.values() {
        let mut new_tile = tile.clone();
        loop {
            let (top_border, left_border) = grid.get_constraint_borders();
            let top_border_ok = if let Some(top_border) = top_border.clone() {
                new_tile.get_border(Border::Top) == top_border
            } else {
                true
            };

            let left_border_ok = if let Some(left_border) = left_border.clone() {
                new_tile.get_border(Border::Left) == left_border
            } else {
                true
            };

            if top_border_ok && left_border_ok {
                if grid.advance(new_tile.clone()) {
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
        let tile_lines = lines.map(|s| s.to_string()).collect::<Vec<String>>();

        let tile = Tile::new(id, tile_lines.try_into().unwrap());
        tiles.insert(id, tile);
    }

    Grid::new(tiles)
}

fn check_for_sea_monsters(mut tile: Tile) -> usize {
    let total_pounds = tile.rows.iter().fold(0, |acc, row| {
        acc + row.chars().filter(|c| *c == '#').count()
    });
    println!("{}", tile.rows.len());
    println!("{}", tile.rows[0].len());

    let mut sea_monster_pounds: HashSet<(usize, usize)> = HashSet::new();

    loop {
        for i in 0..tile.rows.len() - 2 {
            let row_one = &tile.rows[i];
            let row_two = &tile.rows[i + 1];
            let row_three = &tile.rows[i + 2];
            for j in 0..row_one.chars().count() - 19 {
                let mut chars_one = row_one.trim().chars();
                let row_one_symbols = [chars_one.nth(18 + j).unwrap()];
                let mut chars_two = row_two.trim().chars();
                let row_two_symbols = [
                    chars_two.nth(0 + j).unwrap(),
                    chars_two.nth(4).unwrap(),
                    chars_two.nth(0).unwrap(),
                    chars_two.nth(4).unwrap(),
                    chars_two.nth(0).unwrap(),
                    chars_two.nth(4).unwrap(),
                    chars_two.nth(0).unwrap(),
                    chars_two.nth(0).unwrap(),
                ];
                let mut chars_three = row_three.trim().chars();
                let row_three_symbols = [
                    chars_three.nth(1 + j).unwrap(),
                    chars_three.nth(2).unwrap(),
                    chars_three.nth(2).unwrap(),
                    chars_three.nth(2).unwrap(),
                    chars_three.nth(2).unwrap(),
                    chars_three.nth(2).unwrap(),
                ];

                if row_one_symbols.len() == row_one_symbols.iter().filter(|&c| *c == '#').count()
                    && row_two_symbols.len()
                        == row_two_symbols.iter().filter(|&c| *c == '#').count()
                    && row_three_symbols.len()
                        == row_three_symbols.iter().filter(|&c| *c == '#').count()
                {
                    println!("{:?} {:?} {:?}", row_one_symbols, row_two_symbols, row_three_symbols);
                    println!("Found a monster {:?} {} {}", tile.symmetry, i, j);
                    sea_monster_pounds.insert((i, 18 + j));

                    sea_monster_pounds.insert((i + 1, 19 + j));
                    sea_monster_pounds.insert((i + 1, 18 + j));
                    sea_monster_pounds.insert((i + 1, 17 + j));
                    sea_monster_pounds.insert((i + 1, 12 + j));
                    sea_monster_pounds.insert((i + 1, 11 + j));
                    sea_monster_pounds.insert((i + 1, 6 + j));
                    sea_monster_pounds.insert((i + 1, 5 + j));
                    sea_monster_pounds.insert((i + 1, 0 + j));

                    sea_monster_pounds.insert((i + 2, 16 + j));
                    sea_monster_pounds.insert((i + 2, 13 + j));
                    sea_monster_pounds.insert((i + 2, 10 + j));
                    sea_monster_pounds.insert((i + 2, 7 + j));
                    sea_monster_pounds.insert((i + 2, 4 + j));
                    sea_monster_pounds.insert((i + 2, 1 + j));
                }
            }
        }

        if sea_monster_pounds.len() != 0 {
            for row in tile.rows {
                println!("{}", row);
            }
            break;
        }

        tile.act();
        if tile.symmetry == Symmetry::I {
            break;
        }
    }

    println!("{}", total_pounds);
    total_pounds - sea_monster_pounds.len()
}
