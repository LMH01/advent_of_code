use std::{cmp::Ordering, collections::HashMap};

aoc::parts!(1, 2);

fn part_1(input: aoc::Input) -> impl ToString {
    run_solution(input, false)
}

fn part_2(input: aoc::Input) -> impl ToString {
    run_solution(input, true)
}

fn run_solution(input: aoc::Input, part_two: bool) -> impl ToString {
    let mut hands = Vec::new();
    for line in input {
        hands.push(Hand::try_from((line, part_two)).unwrap());
    }

    // hands are sorted in ascending order using the trait implementations below
    hands.sort();

    //for hand in &hands {
    //    println!("{:?}", hand);
    //}

    let mut result = 0;
    for (idx, hand) in hands.iter().enumerate() {
        result += (idx + 1) * (hand.bid as usize);
        //println!("Result: {} - {}", result, hand.bid);
    }
    result
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    hand_type: Option<HandType>,
    /// If part two is enabled, jacks are treated as jokers and but will individually be the weakest card
    part_two: bool,
}

impl Hand {
    fn calc_hand_type(&self) -> HandType {
        let mut cards = HashMap::new();
        // function only works if we have exactly 5 cards
        if self.cards.len() > 5 {
            // we panic because I don't see a good reason to do proper error handling here
            panic!("Hand type was called while hand contains more than 5 cards!");
        }
        // Determine how many cards of what type we have
        for card in self.cards.iter() {
            if cards.contains_key(card) {
                let new_value = cards.get(card).unwrap() + 1;
                cards.insert(card, new_value);
            } else {
                cards.insert(card, 1);
            }
        }
        // we have to store if we have found a three of a kind to check for full houses
        let mut three_of_a_kind_found = false;
        let jokers = match self.part_two {
            true => *cards.get(&Card::J(true)).unwrap_or(&0),
            false => 0,
        };
        let mut threes = 0;
        // collect pairs that have been found
        let mut pairs = 0;
        for (card, amount) in cards.iter() {
            // skip joker card except when it matches an amount of 5 cards
            if **card == Card::J(true) {
                if *amount == 5 {
                    return HandType::FiveOfAKind;
                }
                continue;
            }
            let amount = amount + jokers;
            match amount {
                5 => return HandType::FiveOfAKind,
                4 => return HandType::FourOfAKind,
                3 => {
                    if pairs == 1 && jokers == 0 {
                        // joker cases are handled below
                        return HandType::FullHouse;
                    }
                    threes += 1;
                    three_of_a_kind_found = true
                }
                2 => {
                    if three_of_a_kind_found && jokers == 0 {
                        // joker cases are handled below
                        return HandType::FullHouse;
                    }
                    pairs += 1;
                }
                _ => (),
            }
        }
        // handle three of a kind and full house for joker cases
        if threes == 2 {
            return HandType::FullHouse;
        }
        if threes > 1 {
            return HandType::TheeOfAKind;
        }
        // if three of a kind is set here we know that the deck is not full house,
        // we can return three of a kind
        if three_of_a_kind_found {
            return HandType::TheeOfAKind;
        }
        // check pairs
        match pairs {
            2 => return HandType::TwoPair,
            1 => {
                // Check if joker exists because that one pair would become a three of a kind
                if jokers == 1 {
                    return HandType::TheeOfAKind;
                }
                return HandType::OnePair;
            }
            _ => (),
        };
        // Check if 1 pair is created because of the joker
        if jokers == 1 {
            return HandType::OnePair;
        }
        HandType::HighCard
    }

    fn hand_type(&self) -> HandType {
        if let Some(hand_type) = &self.hand_type {
            return hand_type.clone();
        }
        // only calculate hand type when it is not already set
        self.calc_hand_type()
    }
}

impl TryFrom<(&str, bool)> for Hand {
    type Error = String;

    fn try_from(value: (&str, bool)) -> Result<Self, Self::Error> {
        // chunks[0] = cards
        // chunks[1] = bid
        let chunks = value.0.split(' ').collect::<Vec<&str>>();
        let mut cards = Vec::new();
        for c in chunks[0].chars() {
            cards.push(Card::try_from((c, value.1))?);
        }
        let bid = chunks[1].parse::<u32>().unwrap(); // again too lazy for proper error handling
                                                     // calculate hand type
        let mut hand = Hand {
            cards,
            bid,
            hand_type: None,
            part_two: value.1,
        };
        hand.hand_type = Some(hand.calc_hand_type());
        Ok(hand)
    }
}

impl TryFrom<&str> for Hand {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from((value, false))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.hand_type();
        let other_type = other.hand_type();

        match self_type.cmp(&other_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                // determine which card is stronger
                for (idx, card) in self.cards.iter().enumerate() {
                    match card.partial_cmp(&other.cards[idx]).unwrap() {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        _ => (),
                    }
                }
                // all cards have ben checked, decks are identical
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Specifies
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    TheeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn strength(&self) -> u8 {
        match self {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::TheeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Card {
    A,
    K,
    Q,
    /// Set to true to treat jack as jocker
    J(bool),
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn strength(&self) -> u8 {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J(b) => {
                if *b {
                    0
                } else {
                    11
                }
            }
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

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl TryFrom<(char, bool)> for Card {
    type Error = String;

    fn try_from(value: (char, bool)) -> Result<Self, Self::Error> {
        match value.0 {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J(value.1)),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            _ => Err(String::from("could not map char to chard")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Card, Hand, HandType};

    #[test]
    fn test_hand_from_str() {
        let hand = Hand::try_from("32T3K 765").unwrap();
        assert_eq!(
            hand,
            Hand {
                cards: vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::K],
                bid: 765,
                hand_type: Some(HandType::OnePair),
                part_two: false
            }
        )
    }

    #[test]
    fn test_hand_type_five_of_a_kind() {
        let hand = Hand::try_from("AAAAA 765").unwrap();
        assert_eq!(hand.hand_type(), HandType::FiveOfAKind);
    }

    #[test]
    fn test_hand_type_four_of_a_kind() {
        let hand = Hand::try_from("AAAAJ 765").unwrap();
        assert_eq!(hand.hand_type(), HandType::FourOfAKind)
    }

    #[test]
    fn test_hand_type_full_house() {
        let hand = Hand::try_from("AAAJJ 765").unwrap();
        assert_eq!(hand.hand_type(), HandType::FullHouse)
    }

    #[test]
    fn test_hand_type_three_of_a_kind() {
        let hand = Hand::try_from("AAA2J 765").unwrap();
        assert_eq!(hand.hand_type(), HandType::TheeOfAKind)
    }

    #[test]
    fn test_hand_type_two_pair() {
        let hand = Hand::try_from("AA233 765").unwrap();
        assert_eq!(hand.hand_type(), HandType::TwoPair)
    }

    #[test]
    fn test_hand_type_one_pair() {
        let hand = Hand::try_from("99T2J 765").unwrap();
        assert_eq!(hand.hand_type(), HandType::OnePair)
    }

    #[test]
    fn test_hand_type_high_card() {
        let hand = Hand::try_from("34TJ5 765").unwrap();
        assert_eq!(hand.hand_type(), HandType::HighCard)
    }

    #[test]
    fn test_hand_type_five_of_a_kind_p2() {
        let hand = Hand::try_from(("AAAAJ 765", true)).unwrap();
        assert_eq!(hand.hand_type(), HandType::FiveOfAKind);
    }

    #[test]
    fn test_hand_type_four_of_a_kind_p2() {
        let hand = Hand::try_from(("JAAJ3 765", true)).unwrap();
        assert_eq!(hand.hand_type(), HandType::FourOfAKind);
        let hand = Hand::try_from(("2333J 765", true)).unwrap();
        assert_eq!(hand.hand_type(), HandType::FourOfAKind);
        let hand = Hand::try_from(("233JJ 765", true)).unwrap();
        assert_eq!(hand.hand_type(), HandType::FourOfAKind);
        let hand = Hand::try_from(("23JJJ 765", true)).unwrap();
        assert_eq!(hand.hand_type(), HandType::FourOfAKind);
        let hand = Hand::try_from(("AJA2J 765", true)).unwrap();
        assert_eq!(hand.hand_type(), HandType::FourOfAKind)
    }

    #[test]
    fn test_hand_type_full_house_p2() {
        let hand = Hand::try_from(("AJA22 765", true)).unwrap();
        assert_eq!(hand.hand_type(), HandType::FullHouse);
        let hand = Hand::try_from(("2233J 765", true)).unwrap();
        assert_eq!(hand.hand_type(), HandType::FullHouse)
    }

    #[test]
    fn test_hand_type_three_of_a_kind_p2() {
        let hand = Hand::try_from(("AJA82 765", true)).unwrap();
        assert_eq!(hand.hand_type(), HandType::TheeOfAKind);
        let hand = Hand::try_from(("AJ233 765", true)).unwrap();
        assert_eq!(hand.hand_type(), HandType::TheeOfAKind);
        let hand = Hand::try_from(("Q2KJJ 765", true)).unwrap();
        assert_eq!(hand.hand_type(), HandType::TheeOfAKind);
        let hand = Hand::try_from(("2344J 765", true)).unwrap();
        assert_eq!(hand.hand_type(), HandType::TheeOfAKind);
        let hand = Hand::try_from(("234JJ 765", true)).unwrap();
        assert_eq!(hand.hand_type(), HandType::TheeOfAKind);
        let hand = Hand::try_from(("AJJ94 765", true)).unwrap();
        assert_eq!(hand.hand_type(), HandType::TheeOfAKind)
    }

    #[test]
    fn test_hand_type_one_pair_p2() {
        let hand = Hand::try_from(("9KT2J 765", true)).unwrap();
        assert_eq!(hand.hand_type(), HandType::OnePair);
        let hand = Hand::try_from(("2345J 765", true)).unwrap();
        assert_eq!(hand.hand_type(), HandType::OnePair)
    }

    #[test]
    fn test_hand_partial_ord() {
        let hand_1 = Hand::try_from("KA2KJ 1").unwrap();
        let hand_2 = Hand::try_from("32AA2 2").unwrap();
        let hand_3 = Hand::try_from("3A2KJ 3").unwrap();
        let mut hands = vec![hand_1, hand_2, hand_3];
        println!("{:?}\n", hands);
        hands.sort();
        println!("{:?}", hands);

        assert_eq!(hands[0].bid, 3);
        assert_eq!(hands[1].bid, 1);
        assert_eq!(hands[2].bid, 2);
    }

    #[test]
    fn test_hand_partial_ord_p2() {
        let hand_1 = Hand::try_from(("AJJ94 2", true)).unwrap();
        let hand_2 = Hand::try_from(("A2223 1", true)).unwrap();
        let hand_3 = Hand::try_from(("KKK23 3", true)).unwrap();
        let mut hands = vec![hand_1, hand_2, hand_3];
        //println!("{:?}\n", hands);
        for hand in &hands {
            println!("{:?}", hand);
        }
        hands.sort();
        for hand in &hands {
            println!("{:?}", hand);
        }
        //println!("{:?}", hands);

        assert_eq!(hands[0].bid, 3);
        assert_eq!(hands[1].bid, 2);
        assert_eq!(hands[2].bid, 1);
    }
}
