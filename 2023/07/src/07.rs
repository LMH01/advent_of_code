use std::collections::HashMap;

aoc::parts!(1);

fn part_1(input: aoc::Input) -> impl ToString {
    0
}

// fn part_2(input: aoc::Input) -> impl ToString {
//     0
// }

struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {

    fn hand_type(&self) -> HandType {
        let mut cards = HashMap::new();
        // Determine how many cards of what type we have
        for card in self.cards.iter() {
            if cards.contains_key(card) {
                let new_value = cards.get(card).unwrap() + 1;
                cards.insert(card, new_value);
            }
        }
        todo!()
    }
}

/// Specifies 
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    TheeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(PartialEq, Eq, Hash)]
enum Card {
    A,
    K,
    Q,
    J,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}

impl Card {

    fn strength(&self) -> u8 {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.strength().cmp(&other.strength()))
    }
}