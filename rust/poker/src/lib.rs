#![feature(map_into_keys_values)]

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
    LowAce = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Jack = 10,
    Queen = 11,
    King = 12,
    HighAce = 13,
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
            "A" => Value::HighAce,
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
    order_by_rank_cards(&lhs.cards, &rhs.cards)
}

fn order_by_rank_cards(lhs: &[Card], rhs: &[Card]) -> Option<Ordering> {
    order_by_rank_values(
        lhs.iter().map(|card| card.value),
        rhs.iter().map(|card| card.value),
    )
}

fn order_by_rank_values(
    lhs: impl Iterator<Item = Value>,
    rhs: impl Iterator<Item = Value>,
) -> Option<Ordering> {
    lhs.zip(rhs)
        .map(|(left, right)| left.partial_cmp(&right).unwrap_or(Ordering::Equal))
        .find(|&ordering| ordering != Ordering::Equal)
}

fn order_by_rank_grouped(lhs: &Hand, rhs: &Hand, cmp_order: &[usize]) -> Option<Ordering> {
    let mut lhs_accounting = count_values(lhs);
    let mut rhs_accounting = count_values(rhs);

    for &order in cmp_order.iter() {
        match lhs_accounting
            .remove(&order)
            .unwrap()
            .cmp(&rhs_accounting.remove(&order).unwrap())
        {
            Ordering::Equal => continue,
            otherwise => return Some(otherwise),
        }
    }

    let lhs_cards = lhs_accounting.into_values().collect::<Vec<_>>();
    let rhs_cards = rhs_accounting.into_values().collect::<Vec<_>>();

    order_by_rank_values(lhs_cards.into_iter(), rhs_cards.into_iter())
}

fn has_card_value(cards: &[Card], value: Value) -> bool {
    cards.iter().any(|card| card.value == value)
}

fn sort_cards(cards: &mut [Card]) {
    cards.sort_by(|left, right| right.partial_cmp(left).unwrap_or(Ordering::Equal));
}

fn ace_low_straight_rank(cards: &[Card]) -> Vec<Card> {
    let mut result = cards.to_owned();
    result.iter_mut().for_each(|card| {
        if card.value == Value::HighAce {
            card.value = Value::LowAce;
        }
    });
    sort_cards(&mut result);
    result
}

fn order_by_rank_straight(lhs: &[Card], rhs: &[Card]) -> Option<Ordering> {
    let lhs = match (
        has_card_value(lhs, Value::HighAce),
        has_card_value(lhs, Value::Two),
        has_card_value(lhs, Value::King),
    ) {
        (true, true, false) => ace_low_straight_rank(lhs),
        _ => lhs.to_owned(),
    };
    let rhs = match (
        has_card_value(rhs, Value::HighAce),
        has_card_value(rhs, Value::Two),
        has_card_value(rhs, Value::King),
    ) {
        (true, true, false) => ace_low_straight_rank(rhs),
        _ => rhs.to_owned(),
    };
    order_by_rank_cards(&lhs, &rhs)
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match (self, other) {
            (Self::StraightFlush(lhs), Self::StraightFlush(rhs)) => {
                return order_by_rank(&lhs, &rhs);
            }
            (Self::FourOfAKind(lhs), Self::FourOfAKind(rhs)) => {
                return order_by_rank_grouped(&lhs, &rhs, &[4, 1]);
            }
            (Self::FullHouse(lhs), Self::FullHouse(rhs)) => {
                return order_by_rank_grouped(&lhs, &rhs, &[3, 2]);
            }
            (Self::Flush(lhs), Self::Flush(rhs)) => return order_by_rank(&lhs, &rhs),
            (Self::Straight(lhs), Self::Straight(rhs)) => {
                return order_by_rank_straight(&lhs.cards, &rhs.cards);
            }
            (Self::ThreeOfAKind(lhs), Self::ThreeOfAKind(rhs)) => {
                return order_by_rank(&lhs, &rhs);
            }
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

fn value_counts(hand: &Hand) -> HashMap<Value, usize> {
    let mut value_map = HashMap::new();

    for &Card { value, .. } in hand.cards.iter() {
        *value_map.entry(value).or_default() += 1;
    }
    value_map
}

fn count_values(hand: &Hand) -> HashMap<usize, Value> {
    value_counts(hand)
        .into_iter()
        .map(|(key, value)| (value, key))
        .collect()
}

fn straight_flush(hand: Hand) -> Option<HandRank> {
    if straight(hand.clone()).is_some() && flush(hand.clone()).is_some() {
        Some(HandRank::StraightFlush(hand))
    } else {
        None
    }
}

fn four_of_a_kind(hand: Hand) -> Option<HandRank> {
    if value_counts(&hand).values().any(|&count| count == 4) {
        Some(HandRank::FourOfAKind(hand))
    } else {
        None
    }
}

fn full_house(hand: Hand) -> Option<HandRank> {
    let full_house_criterion = vec![2, 3].into_iter().collect();
    if value_counts(&hand)
        .values()
        .copied()
        .collect::<HashSet<_>>()
        == full_house_criterion
    {
        Some(HandRank::FullHouse(hand))
    } else {
        None
    }
}

fn flush(hand: Hand) -> Option<HandRank> {
    if hand
        .cards
        .iter()
        .map(|card| card.suit)
        .collect::<HashSet<_>>()
        .len()
        == 1
    {
        Some(HandRank::Flush(hand))
    } else {
        None
    }
}

fn is_straight(card_values: &[Value]) -> bool {
    let result = card_values
        .windows(2)
        .map(|window| window[1] - window[0])
        .all(|value| value == 1);
    result
}

fn straight(hand: Hand) -> Option<HandRank> {
    let mut card_values = hand.cards.iter().map(|card| card.value).collect::<Vec<_>>();
    card_values.sort();

    if is_straight(&card_values) {
        Some(HandRank::Straight(hand))
    } else {
        let mut ace_low_card_values = card_values
            .into_iter()
            .map(|value| {
                if value == Value::HighAce {
                    Value::LowAce
                } else {
                    value
                }
            })
            .collect::<Vec<_>>();
        ace_low_card_values.sort();
        if is_straight(&ace_low_card_values) {
            Some(HandRank::Straight(hand))
        } else {
            None
        }
    }
}

fn three_of_a_kind(hand: Hand) -> Option<HandRank> {
    if value_counts(&hand).values().any(|&count| count == 3) {
        Some(HandRank::ThreeOfAKind(hand))
    } else {
        None
    }
}

fn two_pair(hand: Hand) -> Option<HandRank> {
    if value_counts(&hand)
        .values()
        .filter(|&&count| count == 2)
        .count()
        == 2
    {
        Some(HandRank::TwoPair(hand))
    } else {
        None
    }
}

fn one_pair(hand: Hand) -> Option<HandRank> {
    if value_counts(&hand).values().any(|&count| count == 2) {
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
        sort_cards(&mut cards);
        Ok(Hand { cards })
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut order = (0..hands.len()).collect::<Vec<_>>();
    let parsed_hands = hands
        .iter()
        .map(|h| h.parse::<Hand>().unwrap())
        .collect::<Vec<_>>();

    order.sort_by(|&left, &right| {
        parsed_hands[right]
            .clone()
            .rank()
            .partial_cmp(&parsed_hands[left].clone().rank())
            .unwrap_or(Ordering::Equal)
    });

    let best_index = order[0];
    let best = parsed_hands[best_index].clone().rank();

    Some(
        order
            .iter()
            .filter(|&&index| {
                best.partial_cmp(&parsed_hands[index].clone().rank())
                    .unwrap_or(Ordering::Equal)
                    == Ordering::Equal
            })
            .map(|&index| hands[index])
            .collect(),
    )
}
