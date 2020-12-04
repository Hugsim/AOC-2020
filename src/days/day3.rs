use crate::util::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    Tree,
    Open,
}

pub fn eval() {
    let map = make_tile_map();

    let result = count_trees(1, 1, &map)
        * count_trees(3, 1, &map)
        * count_trees(5, 1, &map)
        * count_trees(7, 1, &map)
        * count_trees(1, 2, &map);
    sprint(result);
}

pub fn count_trees(x_slope: usize, y_slope: usize, map: &Vec<Vec<Tile>>) -> i64 {
    let mut x = 0;
    let mut y = 0;

    let mut counter = 0;
    loop {
        if y >= map.len() {
            break;
        }

        if map[y][x] == Tile::Tree {
            counter += 1;
        }

        x = (x + x_slope) % map[0].len();
        y += y_slope;
    }
    counter
}

pub fn make_tile_map() -> Vec<Vec<Tile>> {
    let contents = read_file_to_vec::<String>("src/days/input/3");
    let contents: Vec<Vec<Tile>> = contents
        .into_iter()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '#' => Tile::Tree,
                    '.' => Tile::Open,
                    _ => unreachable!(),
                })
                .collect::<Vec<Tile>>()
        })
        .collect();

    contents
}
