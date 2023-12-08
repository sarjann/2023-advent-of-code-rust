use utils::load;
use part1::part1;
use part2::part2;

fn main() {
    let data = load("input.txt");
    part1(&data);
    part2(&data);
    let data = load("test.txt");
    part1(&data);
    part2(&data);
}
