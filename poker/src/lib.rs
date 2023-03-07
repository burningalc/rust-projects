use std::{
    collections::{HashMap, HashSet},
    fmt,
};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
pub struct HandType {
    pub hand_category: HandCategory,
    pub rank: Vec<Rank>,
}

#[derive(Clone)]
pub struct Hand<'a> {
    pub representation: &'a str,
    pub cards: HashSet<Card>,
    pub hand_type: HandType,
}

impl fmt::Debug for Hand<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut content = String::from("");
        content.push_str(&format!(" ({:?})", (*self.representation).to_string()));
        content.push_str(&format!(" ({:?})", self.hand_type));
        write!(f, "{}", content)
    }
}

impl PartialEq for Hand<'_> {
    fn eq(&self, other: &Hand) -> bool {
        self.cards == other.cards
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Rank {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Suit {
    Diamonds = 1,
    Clubs = 2,
    Hearts = 3,
    Spades = 4,
}
#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
pub enum HandCategory {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let parsed_hands = hands
        .iter()
        .map(|hand| parse_hand(hand).unwrap())
        .collect::<Vec<Hand>>();

    print!("parsed_hands: {:?}", parsed_hands);

    let mut winning_hands: Vec<Hand> = Vec::new();

    for hand in parsed_hands {
        if winning_hands.is_empty() {
            winning_hands.push(hand);
            continue;
        }

        if hand.hand_type == winning_hands.first().unwrap().hand_type {
            winning_hands.push(hand);
            continue;
        }

        if hand.hand_type > winning_hands.first().unwrap().hand_type {
            winning_hands.clear();
            winning_hands.push(hand);
        }
    }

    winning_hands
        .iter()
        .map(|hand| hand.representation)
        .collect()
}

pub fn parse_hand(hand: &str) -> Result<Hand, String> {
    let cards = hand.split(' ').collect::<Vec<&str>>();
    let mut cards = cards
        .iter()
        .map(|card| parse_card(card))
        .collect::<Vec<Card>>();

    // sort the cards
    cards.sort();

    let cards: HashSet<Card> = cards.into_iter().collect();

    // error if there are not 5 unique cards
    if cards.len() != 5 {
        return Err("Hand must have 5 unique cards".to_string());
    }

    let hand_type = find_hand_type(&cards).unwrap();

    Ok(Hand {
        representation: hand,
        cards,
        hand_type,
    })
}

pub fn parse_card(card: &str) -> Card {
    let suit = match card.chars().last().unwrap() {
        'S' => Suit::Spades,
        'H' => Suit::Hearts,
        'D' => Suit::Diamonds,
        'C' => Suit::Clubs,
        _ => panic!("Invalid suit"),
    };
    let mut card = card.to_string();
    card.pop();

    let rank = match card.as_str() {
        "A" => Rank::Ace,
        "K" => Rank::King,
        "Q" => Rank::Queen,
        "J" => Rank::Jack,
        "10" => Rank::Ten,
        "9" => Rank::Nine,
        "8" => Rank::Eight,
        "7" => Rank::Seven,
        "6" => Rank::Six,
        "5" => Rank::Five,
        "4" => Rank::Four,
        "3" => Rank::Three,
        "2" => Rank::Two,
        _ => panic!("Invalid rank"),
    };

    Card { rank, suit }
}

/// Find the hand type of a given hand
/// idea taken from https://stackoverflow.com/questions/10363927/the-simplest-algorithm-for-poker-hand-evaluation
pub fn find_hand_type(cards: &HashSet<Card>) -> Result<HandType, String> {
    // check if there are 5 cards
    if cards.len() != 5 {
        return Err("Hand must have 5 cards".to_string());
    }

    let is_flush = cards
        .iter()
        .all(|card| card.suit == cards.iter().next().unwrap().suit);

    let rank_count: HashMap<Rank, u8> = cards.iter().fold(HashMap::new(), |mut acc, card| {
        *acc.entry(card.rank).or_insert(0) += 1;
        acc
    });

    match rank_count.len() {
        5 => {
            // straight / straight flush / high card / flush case
            let mut sorted_ranks: Vec<Rank> = rank_count.keys().cloned().collect();
            sorted_ranks.sort();

            let mut is_straight = true;
            for (index, rank) in sorted_ranks.iter().enumerate() {
                if index == sorted_ranks.len() - 1 {
                    continue;
                }

                if *rank == Rank::Five && sorted_ranks[index + 1] == Rank::Ace {
                    // for the special case 2 3 4 5 A => A 2 3 4 5
                    continue;
                }

                if *rank as u8 != sorted_ranks[index + 1] as u8 - 1 {
                    is_straight = false;
                    break;
                }
            }

            match (is_straight, is_flush) {
                (true, true) => {
                    // check if it is A 2 3 4 5
                    if *sorted_ranks.last().unwrap() == Rank::Ace {
                        return Ok(HandType {
                            hand_category: HandCategory::StraightFlush,
                            rank: vec![Rank::Five],
                        });
                    }

                    Ok(HandType {
                        hand_category: HandCategory::StraightFlush,
                        rank: vec![*sorted_ranks.last().unwrap()],
                    })
                }
                (true, false) => {
                    // check if it is A 2 3 4 5
                    if *sorted_ranks.last().unwrap() == Rank::Ace {
                        return Ok(HandType {
                            hand_category: HandCategory::Straight,
                            rank: vec![Rank::Five],
                        });
                    }
                    Ok(HandType {
                        hand_category: HandCategory::Straight,
                        rank: vec![*sorted_ranks.last().unwrap()],
                    })
                }
                (false, true) => Ok(HandType {
                    hand_category: HandCategory::Flush,
                    rank: sorted_ranks.into_iter().rev().collect(),
                }),
                (false, false) => Ok(HandType {
                    hand_category: HandCategory::HighCard,
                    rank: sorted_ranks.into_iter().rev().collect(),
                }),
            }
        }
        4 => {
            // one pair case
            Ok(HandType {
                hand_category: HandCategory::OnePair,
                rank: vec![*rank_count
                    .iter()
                    .find_map(|(rank, &count)| if count == 2 { Some(rank) } else { None })
                    .unwrap()],
            })
        }
        3 => {
            // two pair / three of a kind case
            match rank_count.values().filter(|count| **count == 3).count() {
                1 => {
                    let mut sorted_rank: Vec<Rank> = rank_count
                        .iter()
                        .filter(|(_, &count)| count == 3)
                        .map(|(rank, _)| *rank)
                        .collect();

                    let mut kickers = rank_count
                        .iter()
                        .filter(|(_, &count)| count == 1)
                        .map(|(rank, _)| *rank)
                        .collect::<Vec<Rank>>();

                    kickers.sort_by(|a, b| b.cmp(a));

                    sorted_rank.extend(kickers);

                    // three of a kind case
                    Ok(HandType {
                        hand_category: HandCategory::ThreeOfAKind,
                        rank: sorted_rank,
                    })
                }
                0 => {
                    // two pair case
                    let mut two_pair = rank_count.iter().fold(vec![], |mut acc, (rank, &count)| {
                        if count == 2 {
                            acc.push(*rank);
                        }
                        acc
                    });

                    // highest to lowest
                    two_pair.sort_by(|a, b| b.cmp(a));

                    two_pair.push(
                        *rank_count
                            .iter()
                            .find_map(|(rank, &count)| if count == 1 { Some(rank) } else { None })
                            .unwrap(),
                    );
                    Ok(HandType {
                        hand_category: HandCategory::TwoPair,
                        rank: two_pair,
                    })
                }
                _ => {
                    // cheating
                    Err("3 unique ranks should have either 2 pairs/3 of a kind".to_string())
                }
            }
        }
        2 => {
            let mut sorted_rank_count = rank_count
                .iter()
                .map(|(k, v)| (*k, *v))
                .collect::<Vec<(Rank, u8)>>();

            sorted_rank_count.sort_by(|a, b| b.1.cmp(&a.1));

            let sorted_rank = sorted_rank_count
                .iter()
                .map(|(k, _)| *k)
                .collect::<Vec<Rank>>();

            // full house / four of a kind case
            match rank_count.values().filter(|count| **count == 4).count() {
                0 => {
                    // full house case
                    Ok(HandType {
                        hand_category: HandCategory::FullHouse,
                        rank: sorted_rank,
                    })
                }
                1 => {
                    // four of a kind case
                    Ok(HandType {
                        hand_category: HandCategory::FourOfAKind,
                        rank: sorted_rank,
                    })
                }
                _ => {
                    // cheating
                    Err("2 unique ranks should have either 4 of a kind/full house".to_string())
                }
            }
        }
        _ => {
            // cheating
            Err("5 cards must have 2-5 unique ranks".to_string())
        }
    }
}
