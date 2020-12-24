use std::collections::HashMap;
use std::{fs::File, io, io::prelude::*};

// A hexagonal grid is really just a cube

const MOVES: [(i32, i32, i32); 6] = [
    (1, -1, 0),
    (-1, 1, 0),
    (1, 0, -1),
    (-1, 0, 1),
    (0, 1, -1),
    (0, -1, 1),
];

struct Grid {
    x: i32,
    y: i32,
    z: i32,

    // 3d vector of tiles
    tiles: HashMap<(i32, i32, i32), Tile>,
}

#[derive(PartialEq, Clone)]
enum Tile {
    White,
    Black,
}

enum HexagonalMove {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            x: 0,
            y: 0,
            z: 0,
            tiles: HashMap::new(),
        }
    }

    fn reset_pos(&mut self) {
        self.x = 0;
        self.y = 0;
        self.z = 0;
    }

    fn mov(&mut self, mov: HexagonalMove) {
        match mov {
            HexagonalMove::East => {
                self.x += 1;
                self.y -= 1;
            }
            HexagonalMove::SouthEast => {
                self.y -= 1;
                self.z += 1;
            }
            HexagonalMove::SouthWest => {
                self.x -= 1;
                self.z += 1;
            }
            HexagonalMove::West => {
                self.x -= 1;
                self.y += 1;
            }
            HexagonalMove::NorthWest => {
                self.z -= 1;
                self.y += 1;
            }
            HexagonalMove::NorthEast => {
                self.z -= 1;
                self.x += 1;
            }
        }
    }

    fn flip(&mut self) {
        match self.tiles.get_mut(&(self.x, self.y, self.z)) {
            Some(tile) => match tile {
                Tile::White => {
                    *tile = Tile::Black;
                }
                Tile::Black => {
                    *tile = Tile::White;
                }
            },
            // Default white. So if none, it's black after the flip
            None => {
                self.tiles.insert((self.x, self.y, self.z), Tile::Black);
            }
        }
    }

    fn count_black(&self) -> usize {
        self.tiles.values().fold(
            0,
            |acc, tile| if *tile == Tile::Black { acc + 1 } else { acc },
        )
    }

    // A little clumsy, but this isn't production code
    fn apply_day(&mut self) {
        let mut map: HashMap<(i32, i32, i32), Tile> = HashMap::new();
        for (pos, tile) in self.tiles.iter() {
            for mov in MOVES.iter() {
                let neighbors_pos = (pos.0 + mov.0, pos.1 + mov.1, pos.2 + mov.2);
                if !self
                    .tiles
                    .contains_key(&(neighbors_pos.0, neighbors_pos.1, neighbors_pos.2))
                {
                    let (_, black_neighbors) =
                        self.count_neighbors(neighbors_pos.0, neighbors_pos.1, neighbors_pos.2);
                    if black_neighbors == 2 {
                        map.insert(
                            (neighbors_pos.0, neighbors_pos.1, neighbors_pos.2),
                            Tile::Black,
                        );
                    } else {
                        map.insert(
                            (neighbors_pos.0, neighbors_pos.1, neighbors_pos.2),
                            Tile::White,
                        );
                    }
                }
            }

            let (_, black_neighbors) = self.count_neighbors(pos.0, pos.1, pos.2);
            if *tile == Tile::Black && (black_neighbors == 0 || black_neighbors > 2) {
                map.insert((pos.0, pos.1, pos.2), Tile::White);
            } else if *tile == Tile::White && black_neighbors == 2 {
                map.insert((pos.0, pos.1, pos.2), Tile::Black);
            } else {
                map.insert((pos.0, pos.1, pos.2), tile.clone());
            }
        }

        self.tiles = map;
    }

    fn count_neighbors(&self, x: i32, y: i32, z: i32) -> (usize, usize) {
        let mut white_neighbors = 0;
        let mut black_neighbors = 0;

        for mov in MOVES.iter() {
            let neighbors_pos = (x + mov.0, y + mov.1, z + mov.2);
            let neighbor = self
                .tiles
                .get(&(neighbors_pos.0, neighbors_pos.1, neighbors_pos.2));
            match neighbor {
                Some(Tile::White) => {
                    white_neighbors += 1;
                }
                Some(Tile::Black) => {
                    black_neighbors += 1;
                }
                None => {
                    white_neighbors += 1;
                }
            }
        }

        (white_neighbors, black_neighbors)
    }
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    File::open("day24/input.txt")?.read_to_string(&mut buffer)?;
    let mut gr = Grid::new();
    for mut line in buffer.split("\n") {
        line = line.trim();
        let mut prev_waiting = false;
        let mut prev = "";
        for c in line.split("") {
            match c {
                "e" if !prev_waiting => gr.mov(HexagonalMove::East),
                "w" if !prev_waiting => gr.mov(HexagonalMove::West),
                "e" if prev_waiting => {
                    match prev {
                        "s" => gr.mov(HexagonalMove::SouthEast),
                        "n" => gr.mov(HexagonalMove::NorthEast),
                        _ => {}
                    }
                    prev_waiting = false;
                }
                "w" if prev_waiting => {
                    match prev {
                        "s" => gr.mov(HexagonalMove::SouthWest),
                        "n" => gr.mov(HexagonalMove::NorthWest),
                        _ => {}
                    }
                    prev_waiting = false;
                }
                "s" => {
                    prev = c;
                    prev_waiting = true;
                }
                "n" => {
                    prev = c;
                    prev_waiting = true;
                }
                _ => {}
            }
        }
        gr.flip();
        gr.reset_pos();
    }

    println!("Answer 1: {}", gr.count_black());
    for _ in 1..=100 {
        gr.apply_day();
    }

    println!("Answer 2: {}", gr.count_black());
    Ok(())
}
