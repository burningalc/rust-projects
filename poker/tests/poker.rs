use poker::winning_hands;
use std::collections::{hash_set, HashSet};

fn hs_from<'a>(input: &[&'a str]) -> HashSet<&'a str> {
    let mut hs = HashSet::new();
    for item in input.iter() {
        hs.insert(*item);
    }
    hs
}

/// Test that the expected output is produced from the given input
/// using the `winning_hands` function.
///
/// Note that the output can be in any order. Here, we use a HashSet to
/// abstract away the order of outputs.
fn test(input: &[&str], expected: &[&str]) {
    assert_eq!(hs_from(&winning_hands(input)), hs_from(expected))
}

#[test]
fn test_single_hand_always_wins() {
    test(&["4S 5S 7H 8D JC"], &["4S 5S 7H 8D JC"])
}

#[test]
fn test_duplicate_hands_always_tie() {
    let input = &["3S 4S 5D 6H JH", "3S 4S 5D 6H JH", "3S 4S 5D 6H JH"];
    assert_eq!(&winning_hands(input), input)
}

#[test]
fn test_highest_card_of_all_hands_wins() {
    test(
        &["4D 5S 6S 8D 3C", "2S 4C 7S 9H 10H", "3S 4S 5D 6H JH"],
        &["3S 4S 5D 6H JH"],
    )
}

#[test]
fn test_a_tie_has_multiple_winners() {
    test(
        &[
            "4D 5S 6S 8D 3C",
            "2S 4C 7S 9H 10H",
            "3S 4S 5D 6H JH",
            "3H 4H 5C 6C JD",
        ],
        &["3S 4S 5D 6H JH", "3H 4H 5C 6C JD"],
    )
}

#[test]
fn test_high_card_can_be_low_card_in_an_otherwise_tie() {
    // multiple hands with the same high cards, tie compares next highest ranked,
    // down to last card
    test(&["3S 5H 6S 8D 7H", "2S 5D 6D 8C 7S"], &["3S 5H 6S 8D 7H"])
}

#[test]
fn test_one_pair_beats_high_card() {
    test(&["4S 5H 6C 8D KH", "2S 4H 6S 4D JH"], &["2S 4H 6S 4D JH"])
}

#[test]
fn test_highest_pair_wins() {
    test(&["4S 2H 6S 2D JH", "2S 4H 6C 4D JD"], &["2S 4H 6C 4D JD"])
}

#[test]
fn test_two_pairs_beats_one_pair() {
    test(&["2S 8H 6S 8D JH", "4S 5H 4C 8C 5C"], &["4S 5H 4C 8C 5C"])
}

#[test]
fn test_two_pair_ranks() {
    // both hands have two pairs, highest ranked pair wins
    test(&["2S 8H 2D 8D 3H", "4S 5H 4C 8S 5D"], &["2S 8H 2D 8D 3H"])
}

#[test]
fn test_two_pairs_second_pair_cascade() {
    // both hands have two pairs, with the same highest ranked pair,
    // tie goes to low pair
    test(&["2S QS 2C QD JH", "JD QH JS 8D QC"], &["JD QH JS 8D QC"])
}

#[test]
fn test_two_pairs_last_card_cascade() {
    // both hands have two identically ranked pairs,
    // tie goes to remaining card (kicker)
    test(&["JD QH JS 8D QC", "JS QS JC 2D QD"], &["JD QH JS 8D QC"])
}

#[test]
fn test_three_of_a_kind_beats_two_pair() {
    test(&["2S 8H 2H 8D JH", "4S 5H 4C 8S 4H"], &["4S 5H 4C 8S 4H"])
}

#[test]
fn test_three_of_a_kind_ranks() {
    //both hands have three of a kind, tie goes to highest ranked triplet
    test(&["2S 2H 2C 8D JH", "4S AH AS 8C AD"], &["4S AH AS 8C AD"])
}

#[test]
fn test_low_three_of_a_kind_beats_high_two_pair() {
    test(&["2H 2D 2C 8H 5H", "AS AC KS KC 6S"], &["2H 2D 2C 8H 5H"])
}

#[test]
fn test_three_of_a_kind_cascade_ranks() {
    // with multiple decks, two players can have same three of a kind,
    // ties go to highest remaining cards
    test(&["4S AH AS 7C AD", "4S AH AS 8C AD"], &["4S AH AS 8C AD"])
}

#[test]
fn test_straight_beats_three_of_a_kind() {
    test(&["4S 5H 4C 8D 4H", "3S 4D 2S 6D 5C"], &["3S 4D 2S 6D 5C"])
}

#[test]
fn test_aces_can_end_a_straight_high() {
    // aces can end a straight (10 J Q K A)
    test(&["4S 5H 4C 8D 4H", "10D JH QS KD AC"], &["10D JH QS KD AC"])
}

#[test]
fn test_aces_can_start_a_straight_low() {
    // aces can start a straight (A 2 3 4 5)
    test(&["4S 5H 4C 8D 4H", "4D AH 3S 2D 5C"], &["4D AH 3S 2D 5C"])
}

#[test]
fn test_no_ace_in_middle_of_straight() {
    // aces cannot be in the middle of a straight (Q K A 2 3)
    test(&["2C 3D 7H 5H 2S", "QS KH AC 2D 3S"], &["2C 3D 7H 5H 2S"])
}

#[test]
fn test_straight_ranks() {
    // both hands with a straight, tie goes to highest ranked card
    test(&["4S 6C 7S 8D 5H", "5S 7H 8S 9D 6H"], &["5S 7H 8S 9D 6H"])
}

#[test]
fn test_straight_scoring() {
    // even though an ace is usually high, a 5-high straight is the lowest-scoring straight
    test(&["2H 3C 4D 5D 6H", "4S AH 3S 2D 5H"], &["2H 3C 4D 5D 6H"])
}

#[test]
fn test_flush_beats_a_straight() {
    test(&["4C 6H 7D 8D 5H", "2S 4S 5S 6S 7S"], &["2S 4S 5S 6S 7S"])
}

#[test]
fn test_flush_cascade() {
    // both hands have a flush, tie goes to high card, down to the last one if necessary
    test(&["4H 7H 8H 9H 6H", "2S 4S 5S 6S 7S"], &["4H 7H 8H 9H 6H"])
}

#[test]
fn test_full_house_beats_a_flush() {
    test(&["3H 6H 7H 8H 5H", "4S 5C 4C 5D 4H"], &["4S 5C 4C 5D 4H"])
}

#[test]
fn test_full_house_ranks() {
    // both hands have a full house, tie goes to highest-ranked triplet
    test(&["4H 4S 4D 9S 9D", "5H 5S 5D 8S 8D"], &["5H 5S 5D 8S 8D"])
}

#[test]
fn test_full_house_cascade() {
    // with multiple decks, both hands have a full house with the same triplet, tie goes to the pair
    test(&["5H 5S 5D 9S 9D", "5H 5S 5D 8S 8D"], &["5H 5S 5D 9S 9D"])
}

#[test]
fn test_four_of_a_kind_beats_full_house() {
    test(&["4S 5H 4D 5D 4H", "3S 3H 2S 3D 3C"], &["3S 3H 2S 3D 3C"])
}

#[test]
fn test_four_of_a_kind_ranks() {
    // both hands have four of a kind, tie goes to high quad
    test(&["2S 2H 2C 8D 2D", "4S 5H 5S 5D 5C"], &["4S 5H 5S 5D 5C"])
}

#[test]
fn test_four_of_a_kind_cascade() {
    // with multiple decks, both hands with identical four of a kind, tie determined by kicker
    test(&["3S 3H 2S 3D 3C", "3S 3H 4S 3D 3C"], &["3S 3H 4S 3D 3C"])
}

#[test]
fn test_straight_flush_beats_four_of_a_kind() {
    test(&["4S 5H 5S 5D 5C", "7S 8S 9S 6S 10S"], &["7S 8S 9S 6S 10S"])
}

#[test]
fn test_aces_can_end_a_straight_flush_high() {
    // aces can end a straight flush (10 J Q K A)
    test(&["KC AH AS AD AC", "10C JC QC KC AC"], &["10C JC QC KC AC"])
}

#[test]
fn test_aces_can_start_a_straight_flush_low() {
    // aces can start a straight flush (A 2 3 4 5)
    test(&["KS AH AS AD AC", "4H AH 3H 2H 5H"], &["4H AH 3H 2H 5H"])
}

#[test]
fn test_no_ace_in_middle_of_straight_flush() {
    // aces cannot be in the middle of a straight flush (Q K A 2 3)
    test(&["2C AC QC 10C KC", "QH KH AH 2H 3H"], &["2C AC QC 10C KC"])
}

#[test]
fn test_straight_flush_ranks() {
    // both hands have a straight flush, tie goes to highest-ranked card
    test(&["4H 6H 7H 8H 5H", "5S 7S 8S 9S 6S"], &["5S 7S 8S 9S 6S"])
}

#[test]
fn test_straight_flush_scoring() {
    // even though an ace is usually high, a 5-high straight flush is the lowest-scoring straight flush
    test(&["2H 3H 4H 5H 6H", "4D AD 3D 2D 5D"], &["2H 3H 4H 5H 6H"])
}

#[test]
fn test_parse_hand() {
    let hand = "2S 3H 4D 5C 7H";
    let cards = vec![
        poker::Card {
            rank: poker::Rank::Two,
            suit: poker::Suit::Spades,
        },
        poker::Card {
            rank: poker::Rank::Three,
            suit: poker::Suit::Hearts,
        },
        poker::Card {
            rank: poker::Rank::Four,
            suit: poker::Suit::Diamonds,
        },
        poker::Card {
            rank: poker::Rank::Five,
            suit: poker::Suit::Clubs,
        },
        poker::Card {
            rank: poker::Rank::Seven,
            suit: poker::Suit::Hearts,
        },
    ];
    let expected = poker::Hand {
        representation: hand,
        cards: cards.into_iter().collect(),
        hand_type: poker::HandType {
            hand_category: poker::HandCategory::HighCard,
            rank: vec![poker::Rank::Seven],
        },
    };
    assert_eq!(expected, poker::parse_hand(hand).unwrap());
}

#[test]
fn test_sort_cards() {
    let cards = vec![
        poker::Card {
            rank: poker::Rank::Two,
            suit: poker::Suit::Spades,
        },
        poker::Card {
            rank: poker::Rank::Two,
            suit: poker::Suit::Diamonds,
        },
        poker::Card {
            rank: poker::Rank::Three,
            suit: poker::Suit::Hearts,
        },
        poker::Card {
            rank: poker::Rank::Four,
            suit: poker::Suit::Diamonds,
        },
        poker::Card {
            rank: poker::Rank::Ace,
            suit: poker::Suit::Hearts,
        },
        poker::Card {
            rank: poker::Rank::Five,
            suit: poker::Suit::Clubs,
        },
    ];
    let mut sorted_cards = cards.clone();
    sorted_cards.sort();

    let expected_sorted_cards = vec![
        poker::Card {
            rank: poker::Rank::Two,
            suit: poker::Suit::Diamonds,
        },
        poker::Card {
            rank: poker::Rank::Two,
            suit: poker::Suit::Spades,
        },
        poker::Card {
            rank: poker::Rank::Three,
            suit: poker::Suit::Hearts,
        },
        poker::Card {
            rank: poker::Rank::Four,
            suit: poker::Suit::Diamonds,
        },
        poker::Card {
            rank: poker::Rank::Five,
            suit: poker::Suit::Clubs,
        },
        poker::Card {
            rank: poker::Rank::Ace,
            suit: poker::Suit::Hearts,
        },
    ];

    assert_eq!(sorted_cards, expected_sorted_cards);
}

#[test]
fn test_parse_straight() {
    let hand = "2S 3H 4D 5C 6H";
    let expected = poker::HandType {
        hand_category: poker::HandCategory::Straight,
        rank: vec![poker::Rank::Six],
    };

    assert_eq!(expected, poker::parse_hand(hand).unwrap().hand_type);
}

#[test]
fn test_straight_for_a2345() {
    let hand = "AS 2H 3D 4C 5H";
    let expected = poker::HandType {
        hand_category: poker::HandCategory::Straight,
        rank: vec![poker::Rank::Five],
    };

    assert_eq!(expected, poker::parse_hand(hand).unwrap().hand_type);
}

#[test]
fn test_straight_flush() {
    let hand = "2S 3S 4S 5S 6S";
    let expected = poker::HandType {
        hand_category: poker::HandCategory::StraightFlush,
        rank: vec![poker::Rank::Six],
    };

    assert_eq!(expected, poker::parse_hand(hand).unwrap().hand_type);
}

#[test]
fn test_flush() {
    let hand = "2S 3S 5S 7S KS";
    let expected = poker::HandType {
        hand_category: poker::HandCategory::Flush,
        rank: vec![
            poker::Rank::King,
            poker::Rank::Seven,
            poker::Rank::Five,
            poker::Rank::Three,
            poker::Rank::Two,
        ],
    };

    assert_eq!(expected, poker::parse_hand(hand).unwrap().hand_type);
}

#[test]
fn test_high_card() {
    let hand = "2S 3H 6D 5C JH";
    let expected = poker::HandType {
        hand_category: poker::HandCategory::HighCard,
        rank: vec![
            poker::Rank::Jack,
            poker::Rank::Six,
            poker::Rank::Five,
            poker::Rank::Three,
            poker::Rank::Two,
        ],
    };

    assert_eq!(expected, poker::parse_hand(hand).unwrap().hand_type);
}

#[test]
fn test_one_pair() {
    let hand = "2S 2H 6D 5C JH";
    let expected = poker::HandType {
        hand_category: poker::HandCategory::OnePair,
        rank: vec![poker::Rank::Two],
    };

    assert_eq!(expected, poker::parse_hand(hand).unwrap().hand_type);
}

#[test]
fn test_three_of_a_kind() {
    let hand = "3S 3H 3D 5C JH";
    let expected = poker::HandType {
        hand_category: poker::HandCategory::ThreeOfAKind,
        rank: vec![poker::Rank::Three, poker::Rank::Jack, poker::Rank::Five],
    };

    assert_eq!(expected, poker::parse_hand(hand).unwrap().hand_type);
}

#[test]
fn test_two_pair() {
    let hand = "2S 2H 6D 6C JH";
    let expected = poker::HandType {
        hand_category: poker::HandCategory::TwoPair,
        rank: vec![poker::Rank::Six, poker::Rank::Two, poker::Rank::Jack],
    };

    assert_eq!(expected, poker::parse_hand(hand).unwrap().hand_type);
}

#[test]
fn test_full_house() {
    let hand = "2S 2H 2D 6C 6H";
    let expected = poker::HandType {
        hand_category: poker::HandCategory::FullHouse,
        rank: vec![poker::Rank::Two, poker::Rank::Six],
    };

    assert_eq!(expected, poker::parse_hand(hand).unwrap().hand_type);
}

#[test]
fn test_four_of_a_kind() {
    let hand = "2S 2H 2D 2C 6H";
    let expected = poker::HandType {
        hand_category: poker::HandCategory::FourOfAKind,
        rank: vec![poker::Rank::Two, poker::Rank::Six],
    };

    assert_eq!(expected, poker::parse_hand(hand).unwrap().hand_type);
}
