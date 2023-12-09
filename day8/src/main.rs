use std::collections::hash_map::HashMap;
use utils::load;

struct Node {
    left: String,
    right: String,
}

enum Direction {
    Left,
    Right,
}

fn generate_nodes_hash_map(data: String) -> (HashMap<String, Node>, Vec<Direction>, Vec<String>) {
    let mut start_a_nodes: Vec<String> = vec![];

    let mut directions: Vec<Direction> = vec![];
    let mut nodes: HashMap<String, Node> = HashMap::new();
    let mut split_data = data.split("\n\n");
    let head = split_data.next().unwrap();
    for ch in head.chars() {
        match ch {
            'L' => directions.push(Direction::Left),
            'R' => directions.push(Direction::Right),
            _ => panic!("Wrong direction"),
        }
    }
    let body = split_data.next().unwrap();
    for line in body.lines() {
        let mut split_line = line.split(" = (");
        let value = split_line.next().unwrap();
        let rhs = split_line.next().unwrap();
        let rhs = rhs.replace("(", "").replace(")", "");
        let split_rhs = &mut rhs.split(", ");
        let left_value = split_rhs.next().unwrap();
        let right_value = split_rhs.next().unwrap();
        let ending = value.chars().last().unwrap();
        match ending {
            'A' => {
                start_a_nodes.push(value.to_string());
            }
            _ => {}
        }
        nodes.insert(
            value.to_string(),
            Node {
                left: left_value.to_string(),
                right: right_value.to_string(),
            },
        );
    }
    (nodes, directions, start_a_nodes)
}

fn parse_nodes(nodes: &HashMap<String, Node>, directions: &Vec<Direction>) -> i32 {
    let mut count = 0;
    let mut current_node = "AAA".to_string();
    while current_node != "ZZZ" {
        for direction in directions {
            let node = nodes.get(&current_node).unwrap();
            match direction {
                Direction::Left => current_node = node.left.to_string(),
                Direction::Right => current_node = node.right.to_string(),
            }
            count += 1;
            if current_node == "ZZZ" {
                break;
            }
        }
    }
    count
}

fn get_gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        get_gcd(b, a % b)
    }
}

fn get_lcm(values: Vec<u64>) -> u64 {
    let mut lcm = values[0];
    for value in values {
        lcm = lcm * value / get_gcd(lcm, value);
    }

    lcm
}

fn parse_nodes_2(
    nodes: &HashMap<String, Node>,
    directions: &Vec<Direction>,
    start_nodes: Vec<String>,
) -> Vec<u64> {
    let mut first_z_hits: Vec<u64> = vec![];

    for node_str in start_nodes {
        let mut count = 0;
        let mut current_node: String = node_str;
        while current_node.chars().last().unwrap() != 'Z' {
            for direction in directions {
                let node = nodes.get(&current_node).unwrap();
                match direction {
                    Direction::Left => current_node = node.left.to_string(),
                    Direction::Right => current_node = node.right.to_string(),
                }
                count += 1;
                if current_node.chars().last().unwrap() == 'Z' {
                    break;
                }
            }
        }
        first_z_hits.push(count);
    }
    first_z_hits
}

fn main() {
    let data = load("input.txt");
    let (nodes, directions, start_nodes) = generate_nodes_hash_map(data);

    let count_part1 = parse_nodes(&nodes, &directions);
    println!("Part 1: {}", count_part1);

    let count_part2 = parse_nodes_2(&nodes, &directions, start_nodes);

    let lcm_part2 = get_lcm(count_part2);

    println!("Part 2: {}", lcm_part2);
}
