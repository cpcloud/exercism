/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    convert::Infallible,
    str::FromStr,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Value {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl std::ops::Sub for Value {
    type Output = isize;

    fn sub(self, rhs: Self) -> Self::Output {
        self as isize - rhs as isize
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Suit {
    Spades,
    Clubs,
    Diamonds,
    Hearts,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Card {
    value: Value,
    suit: Suit,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.value.cmp(&other.value) {
            Ordering::Equal => None,
            otherwise => Some(otherwise),
        }
    }
}

impl FromStr for Card {
    type Err = Infallible;
    fn from_str(card: &str) -> Result<Self, Self::Err> {
        assert!(card.len() >= 2);
        assert!(card.len() <= 3);

        let value = match &card[..card.len() - 1] {
            "1" => Value::One,
            "2" => Value::Two,
            "3" => Value::Three,
            "4" => Value::Four,
            "5" => Value::Five,
            "6" => Value::Six,
            "7" => Value::Seven,
            "8" => Value::Eight,
            "9" => Value::Nine,
            "10" => Value::Ten,
            "J" => Value::Jack,
            "Q" => Value::Queen,
            "K" => Value::King,
            "A" => Value::Ace,
            s => panic!("invalid face value: {}", s),
        };

        let suit = match card.chars().last().unwrap() {
            'S' => Suit::Spades,
            'C' => Suit::Clubs,
            'D' => Suit::Diamonds,
            'H' => Suit::Hearts,
            c => panic!("invalid suit: {}", c),
        };

        Ok(Card { value, suit })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn rank(self) -> HandRank {
        for check in &[
            straight_flush,
            four_of_a_kind,
            full_house,
            flush,
            straight,
            three_of_a_kind,
            two_pair,
            one_pair,
        ] {
            if let Some(hand_rank) = check(self.clone()) {
                return hand_rank;
            }
        }
        HandRank::HighCard(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum HandRank {
    HighCard(Hand),
    OnePair(Hand),
    TwoPair(Hand),
    ThreeOfAKind(Hand),
    Straight(Hand),
    Flush(Hand),
    FullHouse(Hand),
    FourOfAKind(Hand),
    StraightFlush(Hand),
}

fn order_by_rank(lhs: &Hand, rhs: &Hand) -> Option<Ordering> {
    lhs.cards
        .iter()
        .copied()
        .zip(rhs.cards.iter().copied())
        .map(|(left, right)| left.partial_cmp(&right).unwrap_or(Ordering::Equal))
        .find(|&ordering| ordering != Ordering::Equal)
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Self::StraightFlush(lhs), Self::StraightFlush(rhs)) => {
                return order_by_rank(&lhs, &rhs)
            }
            (Self::FourOfAKind(lhs), Self::FourOfAKind(rhs)) => return order_by_rank(&lhs, &rhs),
            (Self::FullHouse(lhs), Self::FullHouse(rhs)) => return order_by_rank(&lhs, &rhs),
            (Self::Flush(lhs), Self::Flush(rhs)) => return order_by_rank(&lhs, &rhs),
            (Self::Straight(lhs), Self::Straight(rhs)) => return order_by_rank(&lhs, &rhs),
            (Self::ThreeOfAKind(lhs), Self::ThreeOfAKind(rhs)) => return order_by_rank(&lhs, &rhs),
            (Self::TwoPair(lhs), Self::TwoPair(rhs)) => return order_by_rank(&lhs, &rhs),
            (Self::OnePair(lhs), Self::OnePair(rhs)) => return order_by_rank(&lhs, &rhs),
            (Self::HighCard(lhs), Self::HighCard(rhs)) => return order_by_rank(&lhs, &rhs),

            (Self::StraightFlush(_), _) => Ordering::Greater,
            (_, Self::StraightFlush(_)) => Ordering::Less,

            (Self::FourOfAKind(_), _) => Ordering::Greater,
            (_, Self::FourOfAKind(_)) => Ordering::Less,

            (Self::FullHouse(_), _) => Ordering::Greater,
            (_, Self::FullHouse(_)) => Ordering::Less,

            (Self::Flush(_), _) => Ordering::Greater,
            (_, Self::Flush(_)) => Ordering::Less,

            (Self::Straight(_), _) => Ordering::Greater,
            (_, Self::Straight(_)) => Ordering::Less,

            (Self::ThreeOfAKind(_), _) => Ordering::Greater,
            (_, Self::ThreeOfAKind(_)) => Ordering::Less,

            (Self::TwoPair(_), _) => Ordering::Greater,
            (_, Self::TwoPair(_)) => Ordering::Less,

            (Self::OnePair(_), _) => Ordering::Greater,
            (_, Self::OnePair(_)) => Ordering::Less,
        })
    }
}

fn straight_flush(hand: Hand) -> Option<HandRank> {
    if straight(hand.clone()).is_some() && flush(hand.clone()).is_some() {
        Some(HandRank::StraightFlush(hand))
    } else {
        None
    }
}

fn four_of_a_kind(hand: Hand) -> Option<HandRank> {
    let mut value_map = HashMap::<_, usize>::new();

    for card in hand.cards.iter().copied() {
        *value_map.entry(card.value).or_default() += 1;
    }

    if value_map.values().copied().any(|count| count == 4) {
        Some(HandRank::FourOfAKind(hand))
    } else {
        None
    }
}

fn full_house(hand: Hand) -> Option<HandRank> {
    let mut value_map = HashMap::<_, usize>::new();

    for card in hand.cards.iter().copied() {
        *value_map.entry(card.value).or_default() += 1;
    }

    let full_house_criterion = vec![2, 3].into_iter().collect();
    if value_map.values().copied().collect::<HashSet<_>>() == full_house_criterion {
        Some(HandRank::FullHouse(hand))
    } else {
        None
    }
}

fn flush(hand: Hand) -> Option<HandRank> {
    let mut suits = hand
        .cards
        .iter()
        .copied()
        .map(|card| card.suit)
        .collect::<Vec<_>>();
    suits.dedup();

    if suits.len() == 1 {
        Some(HandRank::Flush(hand))
    } else {
        None
    }
}

fn straight(hand: Hand) -> Option<HandRank> {
    let mut cards = hand
        .cards
        .iter()
        .copied()
        .map(|card| card.value)
        .collect::<Vec<_>>();
    cards.sort();

    if cards
        .windows(2)
        .map(|window| window[1] - window[0])
        .all(|value| value == 1)
    {
        Some(HandRank::Straight(hand))
    } else {
        None
    }
}

fn three_of_a_kind(hand: Hand) -> Option<HandRank> {
    let mut value_map = HashMap::<_, usize>::new();

    for card in hand.cards.iter().copied() {
        *value_map.entry(card.value).or_default() += 1;
    }

    if value_map.values().any(|&count| count == 3) {
        Some(HandRank::ThreeOfAKind(hand))
    } else {
        None
    }
}

fn two_pair(hand: Hand) -> Option<HandRank> {
    let mut value_map = HashMap::<_, usize>::new();

    for card in hand.cards.iter().copied() {
        *value_map.entry(card.value).or_default() += 1;
    }

    if value_map.values().filter(|&&count| count == 2).count() == 2 {
        Some(HandRank::TwoPair(hand))
    } else {
        None
    }
}

fn one_pair(hand: Hand) -> Option<HandRank> {
    let mut value_map = HashMap::<_, usize>::new();

    for card in hand.cards.iter().copied() {
        *value_map.entry(card.value).or_default() += 1;
    }

    if value_map.values().any(|&count| count == 2) {
        Some(HandRank::OnePair(hand))
    } else {
        None
    }
}

impl FromStr for Hand {
    type Err = Infallible;

    fn from_str(hand: &str) -> Result<Self, Self::Err> {
        let strings = hand.split(" ").collect::<Vec<_>>();
        let mut cards = strings
            .into_iter()
            .map(|s| s.parse::<Card>())
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(cards.len(), 5);
        cards.sort_by(|left, right| right.partial_cmp(&left).unwrap_or(Ordering::Equal));
        Ok(Hand { cards })
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut order = (0..hands.len()).collect::<Vec<_>>();
    let parsed_hands = hands
        .iter()
        .copied()
        .map(|h| h.parse::<Hand>().unwrap())
        .collect::<Vec<_>>();
    order.sort_by(|&left, &right| {
        parsed_hands[left]
            .rank()
            .partial_cmp(&parsed_hands[right].rank())
            .unwrap_or(Ordering::Equal)
    });

    let mut result = vec![];

    for (left, right) in order.windows(2).copied() {
        if let Ordering::Equal = parsed_hands[left].cmp(&parsed_hands[right]) {
            result.push(hands[left]);
        }
    }

    Some(result)
}
