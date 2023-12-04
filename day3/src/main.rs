use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use std::hash::{Hash, Hasher};
use utils::load;

#[derive(Eq, PartialEq, Copy, Clone)]
struct Point {
    i: i32,
    j: i32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({}, {})", self.i, self.j)
    }
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.i.hash(state);
        self.j.hash(state);
    }
}

struct Grid {
    gear_nums: HashMap<Point, Vec<i32>>,
    array: Vec<Vec<char>>,
    x: i32,
    y: i32,
}

impl Grid {
    fn build_grid(data: &Vec<&str>) -> Grid {
        let mut array: Vec<Vec<char>> = vec![];
        for row in data {
            let row_chars = row.chars().collect();
            array.push(row_chars);
        }
        let x = array[0].len() as i32;
        let y = array.len() as i32;

        Grid {
            array,
            x,
            y,
            gear_nums: HashMap::new(),
        }
    }

    fn is_symbol_and_has_gear(&mut self, i: i32, j: i32) -> (bool, bool) {
        if (i < 0) || (i >= self.x) || (j < 0) || (j >= self.y) {
            return (false, false);
        };

        let ch: char = self.array[j as usize][i as usize];
        return match ch {
            '0'..='9' => (false, false),
            '.' => (false, false),
            '*' => (true, true),
            _ => (true, false),
        };
    }

    fn update_gear_data(&mut self, gears: &Vec<Point>, number: i32 ) {
        for gear in gears {
            let gear_vec = self.gear_nums.get_mut(gear);
            match gear_vec {
                Some(x) => {
                    x.push(number);
                }
                None => {
                    self.gear_nums.insert(gear.clone(), vec![number]);
                }
            }
        }
    }

    fn has_symbol_neighbour(&mut self, i: i32, j: i32) -> (bool, Vec<Point>) {
        let mut gears: Vec<Point> = vec![];
        for di in [-1, 0, 1] {
            let ni = di + i;
            for dj in [-1, 0, 1] {
                if (di == 0) && (dj == 0) {
                    continue;
                }
                let nj = dj + j;
                let (is_symbol, has_gear) = self.is_symbol_and_has_gear(ni, nj);

                // Gear logic
                if has_gear {
                    let point = Point { i: ni, j: nj };
                    gears.push(point);
                }

                if is_symbol {
                    return (true, gears);
                }
            }
        }
        (false, gears)
    }

    fn check_char_string_and_update(
        &mut self,
        valid_buffer: &mut bool,
        char_buffer: &mut Vec<char>,
        valid_int_array: &mut Vec<i32>,
        gears: &Vec<Point>,
    ) {
        if *valid_buffer {
            let mut number: i32 = 0;
            let length = char_buffer.len();

            for (j, c) in char_buffer.iter().enumerate() {
                let int_val = c.to_digit(10).unwrap() as i32;
                number += int_val * 10_i32.pow(length as u32 - j as u32 - 1);
            }
            if gears.len() > 0 {
                self.update_gear_data(gears, number);
            }
            valid_int_array.push(number);
            *valid_buffer = false;
        }
        *char_buffer = vec![];
        char_buffer.clear();
    }
    fn run(&mut self) {
        let mut valid_int_array: Vec<i32> = vec![];
        let mut char_buffer: Vec<char> = vec![];
        let mut valid_buffer: bool = false;
        let mut gears: Vec<Point> = vec![];
        let array = self.array.clone();

        for (j, row) in array.iter().enumerate() {
            for (i, ch) in row.iter().enumerate() {
                match ch {
                    '0'..='9' => {
                        char_buffer.push(*ch);
                        if !valid_buffer {
                            let (vb, new_gears) = self.has_symbol_neighbour(i as i32, j as i32);
                            valid_buffer = vb;
                            let mut new_gears = new_gears;
                            gears.append(&mut new_gears);
                        };
                    }
                    _ => {
                        self.check_char_string_and_update(
                            &mut valid_buffer,
                            &mut char_buffer,
                            &mut valid_int_array,
                            &gears,
                        );
                        gears.clear();
                    }
                }
            }
        }

        self.check_char_string_and_update(
            &mut valid_buffer,
            &mut char_buffer,
            &mut valid_int_array,
            &gears,
        );
        gears.clear();

        let total: i32 = valid_int_array.iter().sum();
        println!("Part 1 sum is: {}", total);
    }
}

fn main() {
    let input = load("input.txt");
    let data = input.lines().collect::<Vec<&str>>();

    let mut grid = Grid::build_grid(&data);
    grid.run();

    let maps = grid.gear_nums;
    let sum_gears: i32 = maps.iter().filter(|(_, v)| {v.len() == 2}).
        map(|(_, v)| {v[0]*v[1]}).sum();
    println!("Part 2 sum is: {}", sum_gears)
}
