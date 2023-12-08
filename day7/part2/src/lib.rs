use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Number(u64),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Outcome {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
    None,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    bid: u64,
    cards: Vec<Card>,
    rank: u64,
    outcome: Option<Outcome>,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_outcome = self.outcome.clone().unwrap();
        let other_outcome = other.outcome.clone().unwrap();
        let self_value = get_outcome_value(&self_outcome);
        let other_value = get_outcome_value(&other_outcome);
        if self_value == other_value {
            for i in 0..=5 {
                let self_card = &self.cards[i];
                let other_card = &other.cards[i];
                let self_card_value = get_token_value(&self_card);
                let other_card_value = get_token_value(&other_card);
                if self_card_value == other_card_value {
                    continue;
                } else {
                    return self_card_value.cmp(&other_card_value);
                }
            }
        }
        self.rank.cmp(&other.rank)
    }
}

impl Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let outcome = match self {
            Outcome::FiveOfAKind => "Five of a kind",
            Outcome::FourOfAKind => "Four of a kind",
            Outcome::FullHouse => "Full house",
            Outcome::ThreeOfAKind => "Three of a kind",
            Outcome::TwoPair => "Two pair",
            Outcome::OnePair => "One pair",
            Outcome::HighCard => "High card",
            Outcome::None => "None",
        };
        write!(f, "{}", outcome)
    }
}

fn get_outcome_value(outcome: &Outcome) -> u64 {
    match outcome {
        Outcome::FiveOfAKind => 7,
        Outcome::FourOfAKind => 6,
        Outcome::FullHouse => 5,
        Outcome::ThreeOfAKind => 4,
        Outcome::TwoPair => 3,
        Outcome::OnePair => 2,
        Outcome::HighCard => 1,
        Outcome::None => 0,
    }
}

fn get_token_value(token: &Card) -> u64 {
    match token {
        Card::Ace => 14,
        Card::King => 13,
        Card::Queen => 12,
        Card::Jack => 1,
        Card::Ten => 10,
        Card::Number(n) => *n,
    }
}

fn parse_data(input: String) -> Vec<Hand> {
    let mut hands: Vec<Hand> = vec![];
    for line in input.lines() {
        let mut split_line = line.split(" ");
        let cards_text = split_line.next().unwrap();
        let bid: u64 = split_line.next().unwrap().parse::<u64>().unwrap();
        let mut cards: Vec<Card> = vec![];

        for ch in cards_text.chars() {
            let card = match ch {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => Card::Jack,
                'T' => Card::Ten,
                _ => Card::Number(ch.to_digit(10).unwrap().into()),
            };
            cards.push(card);
        }
        let hand = Hand {
            bid,
            cards,
            rank: 0,
            outcome: None,
        };
        hands.push(hand);
    }
    hands
}

fn run_analysis(hands: Vec<Hand>) -> Vec<Hand> {
    let mut outcomes: Vec<Outcome> = vec![];
    let mut ranked_hands = hands;
    for hand in &ranked_hands {
        let mut card_hash: HashMap<Card, u64> = HashMap::new();
        for card in &hand.cards {
            let count = card_hash.entry(card.clone()).or_insert(0);
            *count += 1;
        }

        let mut high_card_value = 0;
        let mut highest_outcome: Outcome = Outcome::None;
        for (card, count) in card_hash.iter() {
            if let Card::Jack = card {
                continue;
            }
            if get_token_value(&card) > high_card_value {
                high_card_value = get_token_value(&card);
            }
            if *count == 5 {
                highest_outcome = Outcome::FiveOfAKind;
            } else if *count == 4 {
                highest_outcome = Outcome::FourOfAKind;
            } else if *count == 3 {
                highest_outcome = match highest_outcome {
                    Outcome::OnePair => Outcome::FullHouse,
                    _ => Outcome::ThreeOfAKind,
                };
            } else if *count == 2 {
                highest_outcome = match highest_outcome {
                    Outcome::OnePair => Outcome::TwoPair,
                    Outcome::ThreeOfAKind => Outcome::FullHouse,
                    _ => Outcome::OnePair,
                };
            } else {
                highest_outcome = match highest_outcome {
                    Outcome::None => Outcome::HighCard,
                    _ => highest_outcome,
                };
            }
        }
        let jack_count: u64 = match card_hash.get(&Card::Jack) {
            Some(x) => *x,
            None => 0,
        };

        if jack_count != 0 {
            highest_outcome = match highest_outcome {
                Outcome::None => match jack_count {
                    5 => Outcome::FiveOfAKind,
                    _ => {
                        println!("Hand: {:?}", hand);
                        println!("Jack count: {}", jack_count);
                        panic!("Invalid jack count");
                    },
                },
                Outcome::HighCard => match jack_count {
                    1 => Outcome::OnePair,
                    2 => Outcome::ThreeOfAKind,
                    3 => Outcome::FourOfAKind,
                    4 => Outcome::FiveOfAKind,
                    _ => panic!("Invalid jack count"),
                },
                Outcome::OnePair => match jack_count {
                    1 => Outcome::ThreeOfAKind,
                    2 => Outcome::FourOfAKind,
                    3 => Outcome::FiveOfAKind,
                    _ => panic!("Invalid jack count"),
                },
                Outcome::TwoPair => match jack_count {
                    1 => Outcome::FullHouse,
                    _ => panic!("Invalid jack count"),
                },
                Outcome::ThreeOfAKind => match jack_count {
                    1 => Outcome::FourOfAKind,
                    2 => Outcome::FiveOfAKind,
                    _ => panic!("Invalid jack count"),
                },
                Outcome::FourOfAKind => match jack_count {
                    1 => Outcome::FiveOfAKind,
                    _ => panic!("Invalid jack count"),
                },
                _ => highest_outcome,
            }
        }

        outcomes.push(highest_outcome);
    }

    for (i, hand) in ranked_hands.iter_mut().enumerate() {
        hand.outcome = Some(outcomes[i].clone());
        hand.rank = get_outcome_value(&outcomes[i]);
    }
    ranked_hands.sort();
    ranked_hands
}

pub fn part2(data: &String) {
    let data = data.clone();
    let parsed_data = parse_data(data);
    let ranked_cards = run_analysis(parsed_data);

    let mut total: u64 = 0;
    for (i, hand) in ranked_cards.iter().enumerate() {
        total += (i + 1) as u64 * hand.bid;
    }
    println!("Total: {}", total);
}
