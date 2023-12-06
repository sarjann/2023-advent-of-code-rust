use std::collections::HashMap;
use utils::load;

struct Mapper {
    to: String,
    ranges: Vec<Range>,
}

struct Range {
    dest: u64,
    source: u64,
    length: u64,
}

fn parse_input(input: String) -> (Vec<u64>, HashMap<String, Mapper>) {
    let mut mappers_map: HashMap<String, Mapper> = HashMap::new();
    let (head, tail) = input.split_once("\n").unwrap();
    // Get seeds from head
    let seeds: Vec<u64> = head.split("seeds: ").collect::<Vec<&str>>()[1]
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    // Get mappers from tail
    let converters = tail
        .split_once("\n")
        .unwrap()
        .1
        .split("\n\n")
        .collect::<Vec<&str>>();

    for converter in converters {
        let mut ranges: Vec<Range> = vec![];
        let (chead, ctail) = converter.split_once("\n").unwrap();
        let from_to_iter: Vec<String> = chead
            .split_once(" ")
            .unwrap()
            .0
            .split("-to-")
            .map(|x| x.to_string())
            .collect();

        let (from, to) = (from_to_iter[0].clone(), from_to_iter[1].clone());
        for line in ctail.split("\n") {
            if line == "" {
                continue;
            }

            let vals: Vec<u64> = line
                .split(" ")
                .map(|x| {
                    return x.parse::<u64>().unwrap();
                })
                .collect();

            let range = Range {
                dest: vals[0],
                source: vals[1],
                length: vals[2],
            };
            ranges.push(range);
        }
        mappers_map.insert(
            from.clone(),
            Mapper {
                to: to.clone(),
                ranges,
            },
        );
    }

    (seeds, mappers_map)
}

fn recurse_mappers(mappers: &HashMap<String, Mapper>, source: &str, i: u64) -> u64 {
    let mapper = mappers.get(source).unwrap();
    let mut out_option: Option<u64> = None;
    for range in &mapper.ranges {
        if (i < range.source + range.length) && (i >= range.source) {
            out_option = Some((i - range.source) + range.dest);
            break;
        }
    }

    let o: u64 = match out_option {
        Some(x) => x,
        _ => i,
    };
    if mapper.to == "location" {
        return o;
    }

    recurse_mappers(mappers, &mapper.to , o)
}

fn part1(mappers: &HashMap<String, Mapper>, seeds: &Vec<u64>) -> u64 {
    let mut min: Option<u64> = None;

    for seed in seeds {
        let res = recurse_mappers(&mappers, "seed", *seed);
        if min.is_none() || res < min.unwrap() {
            min = Some(res);
        };
    }
    min.unwrap()
}

fn part2(mappers: &HashMap<String, Mapper>, seeds: &Vec<u64>) -> u64 {
    let mut min: Option<u64> = None;

    let length = seeds.len();
    for i in 0..length/2 {
        let start = seeds[2*i];
        let range_length = seeds[2*i + 1];
        for j in start..start+range_length{
            let res = recurse_mappers(&mappers, "seed", j);
            if min.is_none() || res < min.unwrap() {
                min = Some(res);
            };
        }
    }
    min.unwrap()
}

fn main() {
    // let input = load("test.txt");
    let input = load("input.txt");
    let (seeds, mapper) = parse_input(input);

    let part1_res = part1(&mapper, &seeds);
    let part2_res = part2(&mapper, &seeds);
    println!("Results for Part 1 is: {}", part1_res);
    println!("Results for Part 2 is: {}", part2_res);
}
