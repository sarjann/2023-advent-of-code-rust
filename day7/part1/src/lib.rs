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
    FiveOfAKind(Card),
    FourOfAKind(Card),
    FullHouse(Card, Card),
    ThreeOfAKind(Card),
    TwoPair(Card, Card),
    OnePair(Card),
    HighCard(Card),
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
            Outcome::FiveOfAKind(_) => "Five of a kind",
            Outcome::FourOfAKind(_) => "Four of a kind",
            Outcome::FullHouse(_, _) => "Full house",
            Outcome::ThreeOfAKind(_) => "Three of a kind",
            Outcome::TwoPair(_, _) => "Two pair",
            Outcome::OnePair(_) => "One pair",
            Outcome::HighCard(_) => "High card",
        };
        write!(f, "{}", outcome)
    }
}

fn get_outcome_value(outcome: &Outcome) -> u64 {
    match outcome {
        Outcome::FiveOfAKind(_) => 7,
        Outcome::FourOfAKind(_) => 6,
        Outcome::FullHouse(_, _) => 5,
        Outcome::ThreeOfAKind(_) => 4,
        Outcome::TwoPair(_, _) => 3,
        Outcome::OnePair(_) => 2,
        Outcome::HighCard(_) => 1,
    }
}

fn get_token_value(token: &Card) -> u64 {
    match token {
        Card::Ace => 14,
        Card::King => 13,
        Card::Queen => 12,
        Card::Jack => 11,
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

        let mut high_card_value = 2;
        let mut highest_outcome: Outcome = Outcome::HighCard(Card::Number(2));
        for (card, count) in card_hash.iter() {
            if get_token_value(&card) > high_card_value {
                high_card_value = get_token_value(&card);
            }
            if *count == 5 {
                highest_outcome = Outcome::FiveOfAKind(card.clone());
            } else if *count == 4 {
                highest_outcome = Outcome::FourOfAKind(card.clone());
            } else if *count == 3 {
                highest_outcome = match highest_outcome {
                    Outcome::OnePair(x) => {
                        Outcome::FullHouse(card.clone(), x.clone())
                    },
                    _ => Outcome::ThreeOfAKind(card.clone()),
                };
            } else if *count == 2 {
                highest_outcome = match highest_outcome {
                    Outcome::OnePair(x) => Outcome::TwoPair(card.clone(), x),
                    Outcome::ThreeOfAKind(x) => Outcome::FullHouse(x, card.clone()),
                    _ => Outcome::OnePair(card.clone()),
                };
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

pub fn part1(data: &String) {
    let data = data.clone();
    let parsed_data = parse_data(data);
    let ranked_cards = run_analysis(parsed_data);

    let mut total: u64 = 0;
    for (i, hand) in ranked_cards.iter().enumerate() {
        total += (i + 1) as u64 * hand.bid;
    }
    println!("Total: {}", total);

}
