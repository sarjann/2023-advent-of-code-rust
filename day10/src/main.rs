use std::collections::HashSet;
use utils::load;

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
// . is ground; there is no pipe in this tile.

#[derive(Debug)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

fn get_tile(ch: &char) -> Tile {
    match ch {
        '|' => Tile::Vertical,
        '-' => Tile::Horizontal,
        'L' => Tile::NorthEast,
        'J' => Tile::NorthWest,
        '7' => Tile::SouthWest,
        'F' => Tile::SouthEast,
        '.' => Tile::Ground,
        'S' => Tile::Start,
        _ => panic!("Unrecognised Character"),
    }
}

fn get_generate_tile_array_start(data: &String) -> (Vec<Vec<Tile>>, (i64, i64)) {
    let mut start: (i64, i64) = (0, 0);
    let mut tile_array: Vec<Vec<Tile>> = vec![];
    for (j, line) in data.lines().enumerate() {
        let mut row_array: Vec<Tile> = vec![];
        for (i, ch) in line.chars().enumerate() {
            let tile = get_tile(&ch);
            match tile {
                Tile::Start => start = (j as i64, i as i64),
                _ => {}
            }
            row_array.push(tile);
        }
        tile_array.push(row_array);
    }
    (tile_array, start)
}

fn is_compatible(current_tile: &Tile, next_tile: &Tile, direction: &(i64, i64)) -> bool {
    match current_tile {
        Tile::Start => match next_tile {
            Tile::Vertical => direction.0 == 0,
            Tile::Horizontal => direction.1 == 0,
            Tile::NorthEast => direction.0 == 1 || direction.1 == -1,
            Tile::NorthWest => direction.0 == 1 || direction.1 == 1,
            Tile::SouthWest => direction.0 == -1 || direction.1 == 1,
            Tile::SouthEast => direction.0 == -1 || direction.1 == -1,
            _ => true,
        },
        _ => true,
    }
}

fn recurse_path(
    tile_array: &Vec<Vec<Tile>>,
    visited: &mut HashSet<(i64, i64)>,
    in_loop: &mut HashSet<(i64, i64)>,
    visited_nodes: &mut Vec<(i64, i64)>,
    current_node: (i64, i64),
    start_node: (i64, i64),
) -> bool {
    // Check if current node is out of bounds
    if (current_node.0 < 0) || (current_node.1 < 0) {
        return false;
    }

    if (current_node.0 as usize >= tile_array.len())
        || (current_node.1 as usize >= tile_array[0].len())
    {
        return false;
    }
    let token = &tile_array[current_node.0 as usize][current_node.1 as usize];
    match token {
        Tile::Ground => return false,
        _ => {}
    }
    //

    if let Some(_) = visited.get(&current_node) {
        if current_node == start_node {
            return true;
        } else {
            return false;
        }
    } else {
        visited.insert(current_node);
    }

    let directions: Vec<(i64, i64)> = match token {
        Tile::Vertical => {
            vec![(1, 0), (-1, 0)]
        }
        Tile::Horizontal => {
            vec![(0, 1), (0, -1)]
        }
        Tile::NorthEast => {
            vec![(0, 1), (-1, 0)]
        }
        Tile::NorthWest => {
            vec![(-1, 0), (0, -1)]
        }
        Tile::SouthWest => {
            vec![(0, -1), (1, 0)]
        }
        Tile::SouthEast => {
            vec![(1, 0), (0, 1)]
        }
        Tile::Start => {
            vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
        }
        _ => {
            panic!("Unknown Token")
        }
    };

    for direction in directions {
        let new_node: (i64, i64) = (current_node.0 + direction.0, current_node.1 + direction.1);
        let new_tile = &tile_array[new_node.0 as usize][new_node.1 as usize];
        if !is_compatible(&token, &new_tile, &direction) {
            continue;
        }
        visited_nodes.push(new_node);
        if recurse_path(
            &tile_array,
            visited,
            in_loop,
            visited_nodes,
            new_node,
            start_node,
        ) {
            return true;
        };
        visited_nodes.pop();
    }
    false
}

fn get_valid_path(
    tile_array: Vec<Vec<Tile>>,
    start: (i64, i64),
) -> (Vec<(i64, i64)>, HashSet<(i64, i64)>, bool) {
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut in_loop: HashSet<(i64, i64)> = HashSet::new();
    let mut visited_nodes: Vec<(i64, i64)> = vec![];

    let is_valid = recurse_path(
        &tile_array,
        &mut visited,
        &mut in_loop,
        &mut visited_nodes,
        start,
        start,
    );
    (visited_nodes, in_loop, is_valid)
}

fn main() {
    let data = load("input.txt");
    let (tile_array, start) = get_generate_tile_array_start(&data);
    let (visited_nodes, in_loop, valid_path) = get_valid_path(tile_array, start);
    println!("Part 1");
    println!("Valid path: {}", valid_path);
    println!("Visited nodes length: {}", visited_nodes.len());
    println!("Half of visited nodes length: {}", visited_nodes.len() / 2);
    println!("Part 2");
}
