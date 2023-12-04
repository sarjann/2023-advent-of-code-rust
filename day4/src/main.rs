use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use utils::load;

#[derive(Eq, PartialEq, Copy, Clone)]
struct Card {
    id: u32,
    count: u32,
    points: u32,
}

impl Hash for Card {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

fn generate_hash_set(left: &Vec<&str>) -> HashSet<u32> {
    let mut game_hash: HashSet<u32> = HashSet::new();
    for lel in left {
        match *lel {
            " " | "" => {}
            _ => {
                let num = lel.parse::<u32>().unwrap();
                game_hash.insert(num);
            }
        }
    }
    return game_hash;
}

fn get_game_points(line: &str, idx: u32) -> Card {
    let mut count: u32 = 0;
    let split: Vec<&str> = line.split(": ").collect::<Vec<&str>>()[1]
        .split("|")
        .collect();

    let left = split[0].split(" ").collect::<Vec<&str>>();
    let right = split[1].split(" ").collect::<Vec<&str>>();

    let hash_set: HashSet<u32> = generate_hash_set(&left);
    for rel in right {
        match rel {
            " " | "" => {}
            _ => {
                let num = rel.parse::<u32>().unwrap();

                if let Some(_) = hash_set.get(&num) {
                    count += 1;
                }
            }
        }
    }

    let mut points = 0;
    if count > 0 {
        points = (2 as u32).pow(count - 1);
    }
    Card { id: idx, count, points}
}

fn part1(hash_games: &HashMap<u32, Card>) -> u32 {
    let mut score = 0;
    for (_, card) in hash_games.iter() {
        score += card.points;
    }
    score
}

fn recurse_card(hash_games: &HashMap<u32, Card>, card: &Card) -> u32 {
    let mut total_cards = 0;
    let i = card.id;
    let count = card.count;
    for j in i+1..(i+count+1) {
        total_cards += 1;
        let card = hash_games.get(&j).unwrap();
        total_cards += recurse_card(&hash_games, &card);
    }
    total_cards
}

fn part2(hash_games: &HashMap<u32, Card>) -> u32 {
    let mut total_cards = 0;
    for (_, card) in hash_games.iter() {
        total_cards += 1;
        total_cards += recurse_card(&hash_games, card);
    }
    total_cards
}

fn main() {
    let mut hash_games: HashMap<u32, Card> = HashMap::new();
    // let data = load("test.txt");
    let data = load("input.txt");

    let mut idx = 0;
    let lines = data.lines();
    for line in lines {
        let card = get_game_points(line, idx);
        hash_games.insert(idx, card);
        idx += 1;
    }

    let p1_score: u32 = part1(&hash_games);
    println!("Total for Part 1 is: {}", p1_score);

    let p2_score: u32 = part2(&mut hash_games);
    println!("Total for Part 2 is: {}", p2_score);
}
