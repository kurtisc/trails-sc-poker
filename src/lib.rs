#[cfg(test)]
mod test;

use crate::Suit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl From<Suit> for u8 {
    fn from(item: Suit) -> Self {
        match item {
            Suit::Club => 0,
            Suit::Diamond => 1,
            Suit::Heart => 2,
            Suit::Spade => 3,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Rank {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl From<u8> for Rank {
    fn from(item: u8) -> Self {
        match item {
            1..=13 => unsafe { std::mem::transmute::<u8, Rank>(item) },
            _ => panic!("Bad rank"),
        }
    }
}

use crate::Rank::*;

#[derive(Clone, PartialEq, Eq)]
pub struct Card(pub Rank, pub Suit);

impl Card {
    pub fn suit(&self) -> &Suit {
        &self.1
    }

    // Rank's discriminants are already 1..=13 in order, so this is a plain
    // load instead of a 13-arm match -- rank() is on the hottest path in
    // the crate (called for every card of every candidate hand).
    pub fn rank(&self) -> usize {
        self.0 as usize
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {

        #[cfg(feature="emoji")]
        let suit = match self.suit() {
            Club => "♣️",
            Diamond => "♦️",
            Heart => "♥️",
            Spade => "♠️",
        };
        #[cfg(not(feature="emoji"))]
        let suit = match self.suit() {
            Club => "♣",
            Diamond => "♦",
            Heart => "♥",
            Spade => "♠",
        };

        let rank = match self.rank() {
            1 => "A",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            10 => "10",
            11 => "J",
            12 => "Q",
            13 => "K",
            _ => panic!(),
        };

        write!(f, "{}{}", rank, suit)
    }
}

impl From<&&Card> for u8 {
    fn from(item: &&Card) -> Self {
        match item {
            Card(Ace, Club) => 0,
            Card(Two, Club) => 1,
            Card(Three, Club) => 2,
            Card(Four, Club) => 3,
            Card(Five, Club) => 4,
            Card(Six, Club) => 5,
            Card(Seven, Club) => 6,
            Card(Eight, Club) => 7,
            Card(Nine, Club) => 8,
            Card(Ten, Club) => 9,
            Card(Jack, Club) => 10,
            Card(Queen, Club) => 11,
            Card(King, Club) => 12,
            Card(Ace, Diamond) => 13,
            Card(Two, Diamond) => 14,
            Card(Three, Diamond) => 15,
            Card(Four, Diamond) => 16,
            Card(Five, Diamond) => 17,
            Card(Six, Diamond) => 18,
            Card(Seven, Diamond) => 19,
            Card(Eight, Diamond) => 20,
            Card(Nine, Diamond) => 21,
            Card(Ten, Diamond) => 22,
            Card(Jack, Diamond) => 23,
            Card(Queen, Diamond) => 24,
            Card(King, Diamond) => 25,
            Card(Ace, Heart) => 26,
            Card(Two, Heart) => 27,
            Card(Three, Heart) => 28,
            Card(Four, Heart) => 29,
            Card(Five, Heart) => 30,
            Card(Six, Heart) => 31,
            Card(Seven, Heart) => 32,
            Card(Eight, Heart) => 33,
            Card(Nine, Heart) => 34,
            Card(Ten, Heart) => 35,
            Card(Jack, Heart) => 36,
            Card(Queen, Heart) => 37,
            Card(King, Heart) => 38,
            Card(Ace, Spade) => 39,
            Card(Two, Spade) => 40,
            Card(Three, Spade) => 41,
            Card(Four, Spade) => 42,
            Card(Five, Spade) => 43,
            Card(Six, Spade) => 44,
            Card(Seven, Spade) => 45,
            Card(Eight, Spade) => 46,
            Card(Nine, Spade) => 47,
            Card(Ten, Spade) => 48,
            Card(Jack, Spade) => 49,
            Card(Queen, Spade) => 50,
            Card(King, Spade) => 51,
        }
    }
}

const NEW_DECK: [Card; 52] = [
    Card(Ace, Club),
    Card(Two, Club),
    Card(Three, Club),
    Card(Four, Club),
    Card(Five, Club),
    Card(Six, Club),
    Card(Seven, Club),
    Card(Eight, Club),
    Card(Nine, Club),
    Card(Ten, Club),
    Card(Jack, Club),
    Card(Queen, Club),
    Card(King, Club),
    Card(Ace, Diamond),
    Card(Two, Diamond),
    Card(Three, Diamond),
    Card(Four, Diamond),
    Card(Five, Diamond),
    Card(Six, Diamond),
    Card(Seven, Diamond),
    Card(Eight, Diamond),
    Card(Nine, Diamond),
    Card(Ten, Diamond),
    Card(Jack, Diamond),
    Card(Queen, Diamond),
    Card(King, Diamond),
    Card(Ace, Heart),
    Card(Two, Heart),
    Card(Three, Heart),
    Card(Four, Heart),
    Card(Five, Heart),
    Card(Six, Heart),
    Card(Seven, Heart),
    Card(Eight, Heart),
    Card(Nine, Heart),
    Card(Ten, Heart),
    Card(Jack, Heart),
    Card(Queen, Heart),
    Card(King, Heart),
    Card(Ace, Spade),
    Card(Two, Spade),
    Card(Three, Spade),
    Card(Four, Spade),
    Card(Five, Spade),
    Card(Six, Spade),
    Card(Seven, Spade),
    Card(Eight, Spade),
    Card(Nine, Spade),
    Card(Ten, Spade),
    Card(Jack, Spade),
    Card(Queen, Spade),
    Card(King, Spade),
];

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
        Deck {
            cards: NEW_DECK.to_vec(),
        }
    }

    fn size(&self) -> usize {
        self.cards.len()
    }

    // Assume exists exactly once
    // Does not preserve deck order
    // But means it's O(1)
    fn take_card(mut self, card: &Card) -> Self {
        for i in 0..self.cards.len() {
            if self.cards[i] == *card {
                self.cards.swap_remove(i);
                break;
            }
        }
        self
    }
}

impl From<Vec<Card>> for Deck {
    fn from(cards: Vec<Card>) -> Self {
        Deck { cards }
    }
}

impl From<&FullHand> for Deck {
    fn from(full_hand: &FullHand) -> Self {
        let deck = Deck::new();
        let deck = deck.take_card(&full_hand.0);
        let deck = deck.take_card(&full_hand.1);
        let deck = deck.take_card(&full_hand.2);
        let deck = deck.take_card(&full_hand.3);
        deck.take_card(&full_hand.4)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Score {
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
}

const PAIR_SCORE: u16 = 2;
const TWO_PAIR_SCORE: u16 = 3;
const THREE_OF_A_KIND_SCORE: u16 = 4;
const STRAIGHT_SCORE: u16 = 8;
const FLUSH_SCORE: u16 = 10;
const FULL_HOUSE_SCORE: u16 = 15;
const FOUR_OF_A_KIND_SCORE: u16 = 50;
const STRAIGHT_FLUSH_SCORE: u16 = 100;
const ROYAL_FLUSH_SCORE: u16 = 500;

impl From<Score> for u16 {
    fn from(score: Score) -> u16 {
        match score {
            Score::Pair => PAIR_SCORE,
            Score::TwoPair => TWO_PAIR_SCORE,
            Score::ThreeOfAKind => THREE_OF_A_KIND_SCORE,
            Score::Straight => STRAIGHT_SCORE,
            Score::Flush => FLUSH_SCORE,
            Score::FullHouse => FULL_HOUSE_SCORE,
            Score::FourOfAKind => FOUR_OF_A_KIND_SCORE,
            Score::StraightFlush => STRAIGHT_FLUSH_SCORE,
            Score::RoyalFlush => ROYAL_FLUSH_SCORE,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FullHand(pub Card, pub Card, pub Card, pub Card, pub Card);
const HAND_SIZE: usize = size_of::<FullHand>() / size_of::<Card>();

mod check {
    use crate::Card;
    use crate::Score;

    pub(crate) fn check(hand: &[&Card; 5]) -> Option<Score> {
        let mut count = [0u8; 13];
        for card in hand {
            count[card.rank() - 1] += 1;
        }

        // If pair+, can only be pair+
        match pair_plus(&count) {
            p @ Some(_) => p,
            None => match straight(flush(hand), &count) {
                f @ Some(_) => f,
                None => None,
            },
        }
    }

    fn straight(score: Option<Score>, count: &[u8; 13]) -> Option<Score> {
        if let (1, 1, 1, 1) = (count[12], count[11], count[10], count[9]) {
            if count[0] == 1 {
                if score == Some(Score::Flush) {
                    return Some(Score::RoyalFlush);
                } else {
                    return Some(Score::Straight);
                }
            }
            if count[8] == 1 {
                if score == Some(Score::Flush) {
                    return Some(Score::StraightFlush);
                } else {
                    return Some(Score::Straight);
                }
            }
            return score;
        }

        for rank in (4..=12).rev() {
            if count[rank] > 1 {
                return score;
            }

            if count[rank] == 1 {
                return match (
                    count[rank - 1],
                    count[rank - 2],
                    count[rank - 3],
                    count[rank - 4],
                ) {
                    (1, 1, 1, 1) => {
                        if score == Some(Score::Flush) {
                            return Some(Score::StraightFlush);
                        } else {
                            return Some(Score::Straight);
                        }
                    }
                    _ => score,
                };
            }
        }
        score
    }

    fn pair_plus(count: &[u8; 13]) -> Option<Score> {
        let mut total = 0;
        let mut max1 = 1;
        let mut max2 = 1;

        for &i in count {
            total += i;
            if i >= max1 {
                max2 = max1;
                max1 = i;
            } else if i > max2 && i < max1 {
                max2 = i;
            }
            if total == 5 {
                break;
            }
        }

        match max1 {
            1 => None,
            2 => match max2 {
                2 => Some(Score::TwoPair),
                _ => Some(Score::Pair),
            },
            3 => match max2 {
                2 => Some(Score::FullHouse),
                _ => Some(Score::ThreeOfAKind),
            },
            _ => Some(Score::FourOfAKind),
        }
    }

    fn flush(hand: &[&Card; 5]) -> Option<Score> {
        let suit = hand[0].suit();
        for card in hand[1..].iter() {
            if card.suit() != suit {
                return None;
            }
        }
        Some(Score::Flush)
    }
}

pub mod tree_check {
    use crate::Card;
    use crate::Deck;
    use crate::FLUSH_SCORE;
    use crate::FOUR_OF_A_KIND_SCORE;
    use crate::FULL_HOUSE_SCORE;
    use crate::FullHand;
    use crate::HAND_SIZE;
    use crate::PAIR_SCORE;
    use crate::ROYAL_FLUSH_SCORE;
    use crate::STRAIGHT_FLUSH_SCORE;
    use crate::STRAIGHT_SCORE;
    use crate::Score;
    use crate::THREE_OF_A_KIND_SCORE;
    use crate::TWO_PAIR_SCORE;
    use crate::check;

    use std::ops::{Add, AddAssign};

    use itertools::Itertools;
    use rational::Rational;
    use rayon::prelude::*;

    #[derive(Debug, PartialEq)]
    pub enum PartialScore {
        Pair(Rational),
        ThreeOfAKind(Rational),
        TwoPair(Rational),
        Straight(Rational),
        Flush(Rational),
        FullHouse(Rational),
        FourOfAKind(Rational),
        StraightFlush(Rational),
        RoyalFlush(Rational),
    }

    impl PartialScore {
        fn score(&self) -> Rational {
            match self {
                PartialScore::Pair(s) => s * PAIR_SCORE,
                PartialScore::TwoPair(s) => s * TWO_PAIR_SCORE,
                PartialScore::ThreeOfAKind(s) => s * THREE_OF_A_KIND_SCORE,
                PartialScore::Straight(s) => s * STRAIGHT_SCORE,
                PartialScore::Flush(s) => s * FLUSH_SCORE,
                PartialScore::FullHouse(s) => s * FULL_HOUSE_SCORE,
                PartialScore::FourOfAKind(s) => s * FOUR_OF_A_KIND_SCORE,
                PartialScore::StraightFlush(s) => s * STRAIGHT_FLUSH_SCORE,
                PartialScore::RoyalFlush(s) => s * ROYAL_FLUSH_SCORE,
            }
        }
    }

    pub fn check(hand: &[&Card], deck: &Deck) -> Vec<PartialScore> {
        let swaps = HAND_SIZE - hand.len();
        let n = deck.size();
        assert!(swaps <= n);

        // Enumerate C(n, swaps) combinations of deck cards (increasing index
        // order), instead of the swaps! permutations of each -- hand
        // scoring doesn't depend on draw order, so this reaches the same
        // enumeration/outcome ratios while doing up to 5! = 120x less work.
        //
        // Below PARALLEL_SWAPS_THRESHOLD, rayon's per-task scheduling
        // overhead dwarfs the handful of leaf combinations each top-level
        // task would do, so it's run as a plain fold instead: measured on
        // this machine, sequential is ~35x faster at 1 swap (781ns vs
        // 27us) and ~2x faster at 2 swaps (16.2us vs 30.8us), while
        // parallel pulls ahead from 3 swaps on (74.8us vs 254.4us) as
        // there's enough work per top-level task to amortize the overhead.
        let deck_tree = if swaps == 0 {
            check::check(&[hand[0], hand[1], hand[2], hand[3], hand[4]]).into()
        } else if swaps < PARALLEL_SWAPS_THRESHOLD {
            (0..=n - swaps).fold(DeckTree::new(), |mut acc, i| {
                let mut full = [&deck.cards[i]; HAND_SIZE];
                full[..hand.len()].copy_from_slice(hand);
                combine(&mut full, hand.len() + 1, &deck.cards, i + 1, swaps - 1, &mut acc);
                acc
            })
        } else {
            (0..=n - swaps)
                .into_par_iter()
                .fold(DeckTree::new, |mut acc, i| {
                    let mut full = [&deck.cards[i]; HAND_SIZE];
                    full[..hand.len()].copy_from_slice(hand);
                    combine(&mut full, hand.len() + 1, &deck.cards, i + 1, swaps - 1, &mut acc);
                    acc
                })
                .reduce(DeckTree::new, |a, b| a + b)
        };

        deck_tree.into()
    }

    const PARALLEL_SWAPS_THRESHOLD: usize = 3;

    // Fills `full[pos..]` with `remaining` more cards drawn (in increasing
    // index order, i.e. as a combination) from `deck_cards[start..]`,
    // scoring and accumulating each completed hand into `tree`.
    fn combine<'a>(
        full: &mut [&'a Card; HAND_SIZE],
        pos: usize,
        deck_cards: &'a [Card],
        start: usize,
        remaining: usize,
        tree: &mut DeckTree,
    ) {
        if remaining == 0 {
            *tree += check::check(full).into();
            return;
        }

        for i in start..=deck_cards.len() - remaining {
            full[pos] = &deck_cards[i];
            combine(full, pos + 1, deck_cards, i + 1, remaining - 1, tree);
        }
    }

    struct DeckTree {
        enumerations: i32,
        pairs: i32,
        two_pairs: i32,
        three_of_a_kinds: i32,
        straights: i32,
        flushes: i32,
        full_houses: i32,
        four_of_a_kinds: i32,
        straight_flushes: i32,
        royal_flushes: i32,
    }

    impl DeckTree {
        pub(crate) fn new() -> Self {
            DeckTree {
                enumerations: 0,
                pairs: 0,
                two_pairs: 0,
                three_of_a_kinds: 0,
                straights: 0,
                flushes: 0,
                full_houses: 0,
                four_of_a_kinds: 0,
                straight_flushes: 0,
                royal_flushes: 0,
            }
        }
    }

    impl From<DeckTree> for Vec<PartialScore> {
        fn from(deck_tree: DeckTree) -> Self {
            let mut result: Vec<PartialScore> = Vec::with_capacity(8);
            if deck_tree.royal_flushes != 0 {
                result.push(PartialScore::RoyalFlush(Rational::new(
                    deck_tree.royal_flushes,
                    deck_tree.enumerations,
                )));
            }
            if deck_tree.straight_flushes != 0 {
                result.push(PartialScore::StraightFlush(Rational::new(
                    deck_tree.straight_flushes,
                    deck_tree.enumerations,
                )));
            }
            if deck_tree.four_of_a_kinds != 0 {
                result.push(PartialScore::FourOfAKind(Rational::new(
                    deck_tree.four_of_a_kinds,
                    deck_tree.enumerations,
                )));
            }
            if deck_tree.full_houses != 0 {
                result.push(PartialScore::FullHouse(Rational::new(
                    deck_tree.full_houses,
                    deck_tree.enumerations,
                )));
            }
            if deck_tree.flushes != 0 {
                result.push(PartialScore::Flush(Rational::new(
                    deck_tree.flushes,
                    deck_tree.enumerations,
                )));
            }
            if deck_tree.straights != 0 {
                result.push(PartialScore::Straight(Rational::new(
                    deck_tree.straights,
                    deck_tree.enumerations,
                )));
            }
            if deck_tree.three_of_a_kinds != 0 {
                result.push(PartialScore::ThreeOfAKind(Rational::new(
                    deck_tree.three_of_a_kinds,
                    deck_tree.enumerations,
                )));
            }
            if deck_tree.two_pairs != 0 {
                result.push(PartialScore::TwoPair(Rational::new(
                    deck_tree.two_pairs,
                    deck_tree.enumerations,
                )));
            }
            if deck_tree.pairs != 0 {
                result.push(PartialScore::Pair(Rational::new(
                    deck_tree.pairs,
                    deck_tree.enumerations,
                )));
            }

            result
        }
    }

    impl From<Option<Score>> for DeckTree {
        fn from(score: Option<Score>) -> Self {
            match score {
                Some(Score::Pair) => DeckTree {
                    enumerations: 1,
                    pairs: 1,
                    two_pairs: 0,
                    three_of_a_kinds: 0,
                    straights: 0,
                    flushes: 0,
                    full_houses: 0,
                    four_of_a_kinds: 0,
                    straight_flushes: 0,
                    royal_flushes: 0,
                },
                Some(Score::TwoPair) => DeckTree {
                    enumerations: 1,
                    pairs: 0,
                    two_pairs: 1,
                    three_of_a_kinds: 0,
                    straights: 0,
                    flushes: 0,
                    full_houses: 0,
                    four_of_a_kinds: 0,
                    straight_flushes: 0,
                    royal_flushes: 0,
                },
                Some(Score::ThreeOfAKind) => DeckTree {
                    enumerations: 1,
                    pairs: 0,
                    two_pairs: 0,
                    three_of_a_kinds: 1,
                    straights: 0,
                    flushes: 0,
                    full_houses: 0,
                    four_of_a_kinds: 0,
                    straight_flushes: 0,
                    royal_flushes: 0,
                },
                Some(Score::Straight) => DeckTree {
                    enumerations: 1,
                    pairs: 0,
                    two_pairs: 0,
                    three_of_a_kinds: 0,
                    straights: 1,
                    flushes: 0,
                    full_houses: 0,
                    four_of_a_kinds: 0,
                    straight_flushes: 0,
                    royal_flushes: 0,
                },
                Some(Score::Flush) => DeckTree {
                    enumerations: 1,
                    pairs: 0,
                    two_pairs: 0,
                    three_of_a_kinds: 0,
                    straights: 0,
                    flushes: 1,
                    full_houses: 0,
                    four_of_a_kinds: 0,
                    straight_flushes: 0,
                    royal_flushes: 0,
                },
                Some(Score::FullHouse) => DeckTree {
                    enumerations: 1,
                    pairs: 0,
                    two_pairs: 0,
                    three_of_a_kinds: 0,
                    straights: 0,
                    flushes: 0,
                    full_houses: 1,
                    four_of_a_kinds: 0,
                    straight_flushes: 0,
                    royal_flushes: 0,
                },
                Some(Score::FourOfAKind) => DeckTree {
                    enumerations: 1,
                    pairs: 0,
                    two_pairs: 0,
                    three_of_a_kinds: 0,
                    straights: 0,
                    flushes: 0,
                    full_houses: 0,
                    four_of_a_kinds: 1,
                    straight_flushes: 0,
                    royal_flushes: 0,
                },
                Some(Score::StraightFlush) => DeckTree {
                    enumerations: 1,
                    pairs: 0,
                    two_pairs: 0,
                    three_of_a_kinds: 0,
                    straights: 0,
                    flushes: 0,
                    full_houses: 0,
                    four_of_a_kinds: 0,
                    straight_flushes: 1,
                    royal_flushes: 0,
                },
                Some(Score::RoyalFlush) => DeckTree {
                    enumerations: 1,
                    pairs: 0,
                    two_pairs: 0,
                    three_of_a_kinds: 0,
                    straights: 0,
                    flushes: 0,
                    full_houses: 0,
                    four_of_a_kinds: 0,
                    straight_flushes: 0,
                    royal_flushes: 1,
                },
                None => DeckTree {
                    enumerations: 1,
                    pairs: 0,
                    two_pairs: 0,
                    three_of_a_kinds: 0,
                    straights: 0,
                    flushes: 0,
                    full_houses: 0,
                    four_of_a_kinds: 0,
                    straight_flushes: 0,
                    royal_flushes: 0,
                },
            }
        }
    }

    impl AddAssign for DeckTree {
        fn add_assign(&mut self, other: Self) {
            self.enumerations += other.enumerations;
            self.pairs += other.pairs;
            self.two_pairs += other.two_pairs;
            self.three_of_a_kinds += other.three_of_a_kinds;
            self.straights += other.straights;
            self.flushes += other.flushes;
            self.full_houses += other.full_houses;
            self.four_of_a_kinds += other.four_of_a_kinds;
            self.straight_flushes += other.straight_flushes;
            self.royal_flushes += other.royal_flushes;
        }
    }

    impl Add for DeckTree {
        type Output = DeckTree;

        fn add(self, other: Self) -> Self {
            DeckTree {
                enumerations: self.enumerations + other.enumerations,
                pairs: self.pairs + other.pairs,
                two_pairs: self.two_pairs + other.two_pairs,
                three_of_a_kinds: self.three_of_a_kinds + other.three_of_a_kinds,
                straights: self.straights + other.straights,
                flushes: self.flushes + other.flushes,
                full_houses: self.full_houses + other.full_houses,
                four_of_a_kinds: self.four_of_a_kinds + other.four_of_a_kinds,
                straight_flushes: self.straight_flushes + other.straight_flushes,
                royal_flushes: self.royal_flushes + other.royal_flushes,
            }
        }
    }

    fn score(hand: &[&Card], deck: &Deck, multiplier: i32) -> Rational {
        const MAX_MULTIPLIER: i32 = 9999;
        let scores = check(hand, deck);
        scores
            .into_iter()
            .fold(Rational::zero(), |mut sum: Rational, s| {
                sum += std::cmp::min(s.score() * multiplier, MAX_MULTIPLIER.into());
                sum
            })
    }

    fn expected_swap_values<'a>(
        full_hand: &'a FullHand,
        deck: &Deck,
        multiplier: i32
    ) -> Vec<(Vec<&'a Card>, Rational)> {
        vec![
            &full_hand.0,
            &full_hand.1,
            &full_hand.2,
            &full_hand.3,
            &full_hand.4,
        ]
        .into_iter()
        .powerset()
        .map(|keep| (keep.clone(), score(&keep[..], deck, multiplier)))
        .collect()
    }

    // Sorted best-first by expected score. Kept separate from `best_score`
    // (and free of I/O) so callers -- CLI printing, benchmarks, tests -- can
    // use the ranking without paying for or filtering out console output.
    pub fn ranked_swap_values<'a>(
        full_hand: &'a FullHand,
        deck: &Deck,
        multiplier: i32,
    ) -> Vec<(Vec<&'a Card>, Rational)> {
        let mut result = expected_swap_values(full_hand, deck, multiplier);
        result.sort_by(|(_, a), (_, b)| a.cmp(b).reverse());
        result
    }

    pub fn best_score(full_hand: FullHand, deck: &Deck, multiplier: Option<i32>) -> Rational {
        let multiplier = multiplier.unwrap_or(1);
        ranked_swap_values(&full_hand, deck, multiplier)[0].1
    }

    #[test]
    fn test_pair() {
        use super::*;

        let hand = vec![
            &Card(Ace, Heart),
            &Card(Two, Heart),
            &Card(Three, Heart),
            &Card(Four, Diamond),
        ];

        let deck1 = vec![Card(Ace, Diamond)].into();

        let result = tree_check::check(&hand, &deck1);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0], PartialScore::Pair(Rational::one()));

        let deck2 = vec![Card(Ace, Diamond), Card(King, Diamond)].into();

        let result = tree_check::check(&hand, &deck2);

        assert_eq!(result, vec![PartialScore::Pair(Rational::new(1, 2))]);
    }

    #[test]
    fn test_three_of_a_kind() {
        use super::*;

        let hand = vec![
            &Card(Ace, Heart),
            &Card(Ace, Diamond),
            &Card(Three, Heart),
            &Card(Four, Diamond),
        ];
        let deck = vec![Card(Ace, Spade)].into();

        let result = tree_check::check(&hand, &deck);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], PartialScore::ThreeOfAKind(Rational::one()));
    }

    #[test]
    fn test_full_house() {
        use super::*;

        let hand = vec![
            &Card(Ace, Heart),
            &Card(Ace, Diamond),
            &Card(Three, Heart),
            &Card(Three, Diamond),
        ];

        let deck = vec![Card(Four, Diamond), Card(Three, Spade), Card(Ace, Spade)].into();

        let result = tree_check::check(&hand, &deck);
        assert_eq!(
            result,
            vec![
                PartialScore::FullHouse(Rational::new(2, 3)),
                PartialScore::TwoPair(Rational::new(1, 3)),
            ]
        );
    }

    #[test]
    fn test_two_pair() {
        use super::*;

        let hand = vec![&Card(Ace, Heart), &Card(Ace, Diamond), &Card(Three, Heart)];

        let deck = vec![Card(Five, Diamond), Card(Three, Spade), Card(Ace, Spade)].into();

        let result = tree_check::check(&hand, &deck);
        assert_eq!(
            result,
            vec![
                PartialScore::FullHouse(Rational::new(1, 3)),
                PartialScore::ThreeOfAKind(Rational::new(1, 3)),
                PartialScore::TwoPair(Rational::new(1, 3)),
            ]
        );
    }

    #[test]
    fn test_three_swaps() {
        use super::*;

        let hand = vec![&Card(Ace, Heart), &Card(Ace, Diamond)];

        let deck = vec![Card(Four, Diamond), Card(Three, Spade), Card(Ace, Spade)].into();

        let result = tree_check::check(&hand, &deck);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], PartialScore::ThreeOfAKind(Rational::one()));
    }

    #[test]
    fn test_four_swaps() {
        use super::*;

        let hand = vec![&Card(Ace, Heart)];

        let deck = vec![
            Card(Ace, Diamond),
            Card(Four, Diamond),
            Card(Three, Spade),
            Card(Ace, Spade),
        ]
        .into();

        let result = tree_check::check(&hand, &deck);
        assert_eq!(result, vec![PartialScore::ThreeOfAKind(Rational::one()),]);
    }

    #[test]
    fn test_flush_four() {
        use super::*;

        let hand = vec![&Card(Ace, Heart)];

        let deck = vec![
            Card(Three, Heart),
            Card(Four, Heart),
            Card(Five, Heart),
            Card(Six, Heart),
            Card(Seven, Heart),
            Card(Eight, Heart),
            Card(Nine, Heart),
            Card(Ten, Heart),
            Card(Jack, Heart),
            Card(Queen, Heart),
            Card(King, Heart),
        ]
        .into();

        let result = tree_check::check(&hand, &deck);
        assert_eq!(
            result,
            vec![
                PartialScore::RoyalFlush(Rational::new(1, 330)),
                PartialScore::Flush(Rational::new(329, 330)),
            ]
        );
    }

    #[test]
    fn test_dump() {
        use super::*;

        let deck = Deck::new();
        let hand = vec![];

        let result = tree_check::check(&hand, &deck);
        assert_eq!(
            result,
            vec![
                PartialScore::RoyalFlush(Rational::new(1, 649740)),
                PartialScore::StraightFlush(Rational::new(3, 216580)),
                PartialScore::FourOfAKind(Rational::new(1, 4165)),
                PartialScore::FullHouse(Rational::new(6, 4165)),
                PartialScore::Flush(Rational::new(1277, 649740)),
                PartialScore::Straight(Rational::new(5, 1274)),
                PartialScore::ThreeOfAKind(Rational::new(88, 4165)),
                PartialScore::TwoPair(Rational::new(198, 4165)),
                PartialScore::Pair(Rational::new(352, 833)),
            ]
        );
    }

    #[test]
    fn test_score() {
        use super::*;

        let full_hand = FullHand(
            Card(King, Heart),
            Card(Eight, Diamond),
            Card(Three, Diamond),
            Card(Four, Diamond),
            Card(Six, Diamond),
        );

        let deck = (&full_hand).into();

        let result = tree_check::score(
            &[&full_hand.0, &full_hand.1, &full_hand.2, &full_hand.3],
            &deck,
            1
        );

        assert_eq!(result, Rational::new(24, 47));
    }

    #[test]
    fn test_best_score() {
        use super::*;

        let full_hand = FullHand(
            Card(King, Heart),
            Card(Eight, Diamond),
            Card(Three, Diamond),
            Card(Four, Diamond),
            Card(Six, Diamond),
        );

        let deck = (&full_hand).into();

        let result = tree_check::best_score(full_hand, &deck, None);

        assert_eq!(result, Rational::new(114, 47));
    }
}

pub mod parse_input {
    use regex::Regex;

    use crate::Card;
    use crate::FullHand;
    use crate::Rank::*;
    use crate::Suit::*;

    const REGEX: &str = r"((\d*|a|A|j|J|q|Q|k|K)([cCdDhHsS]))(?: |,|\.?)";

    impl TryFrom<(&str, &str)> for Card {
        type Error = ();

        fn try_from((rank, suit): (&str, &str)) -> Result<Self, Self::Error> {
            match suit {
                "C" | "c" => match rank {
                    "1" | "A" | "a" => Ok(Card(Ace, Club)),
                    "2" => Ok(Card(Two, Club)),
                    "3" => Ok(Card(Three, Club)),
                    "4" => Ok(Card(Four, Club)),
                    "5" => Ok(Card(Five, Club)),
                    "6" => Ok(Card(Six, Club)),
                    "7" => Ok(Card(Seven, Club)),
                    "8" => Ok(Card(Eight, Club)),
                    "9" => Ok(Card(Nine, Club)),
                    "10" => Ok(Card(Ten, Club)),
                    "11" | "J" | "j" => Ok(Card(Jack, Club)),
                    "12" | "Q" | "q" => Ok(Card(Queen, Club)),
                    "13" | "K" | "k" => Ok(Card(King, Club)),
                    _ => Err(()),
                },
                "D" | "d" => match rank {
                    "1" | "A" | "a" => Ok(Card(Ace, Diamond)),
                    "2" => Ok(Card(Two, Diamond)),
                    "3" => Ok(Card(Three, Diamond)),
                    "4" => Ok(Card(Four, Diamond)),
                    "5" => Ok(Card(Five, Diamond)),
                    "6" => Ok(Card(Six, Diamond)),
                    "7" => Ok(Card(Seven, Diamond)),
                    "8" => Ok(Card(Eight, Diamond)),
                    "9" => Ok(Card(Nine, Diamond)),
                    "10" => Ok(Card(Ten, Diamond)),
                    "11" | "J" | "j" => Ok(Card(Jack, Diamond)),
                    "12" | "Q" | "q" => Ok(Card(Queen, Diamond)),
                    "13" | "K" | "k" => Ok(Card(King, Diamond)),
                    _ => Err(()),
                },
                "h" | "H" => match rank {
                    "1" | "A" | "a" => Ok(Card(Ace, Heart)),
                    "2" => Ok(Card(Two, Heart)),
                    "3" => Ok(Card(Three, Heart)),
                    "4" => Ok(Card(Four, Heart)),
                    "5" => Ok(Card(Five, Heart)),
                    "6" => Ok(Card(Six, Heart)),
                    "7" => Ok(Card(Seven, Heart)),
                    "8" => Ok(Card(Eight, Heart)),
                    "9" => Ok(Card(Nine, Heart)),
                    "10" => Ok(Card(Ten, Heart)),
                    "11" | "J" | "j" => Ok(Card(Jack, Heart)),
                    "12" | "Q" | "q" => Ok(Card(Queen, Heart)),
                    "13" | "K" | "k" => Ok(Card(King, Heart)),
                    _ => Err(()),
                },
                "s" | "S" => match rank {
                    "1" | "A" | "a" => Ok(Card(Ace, Spade)),
                    "2" => Ok(Card(Two, Spade)),
                    "3" => Ok(Card(Three, Spade)),
                    "4" => Ok(Card(Four, Spade)),
                    "5" => Ok(Card(Five, Spade)),
                    "6" => Ok(Card(Six, Spade)),
                    "7" => Ok(Card(Seven, Spade)),
                    "8" => Ok(Card(Eight, Spade)),
                    "9" => Ok(Card(Nine, Spade)),
                    "10" => Ok(Card(Ten, Spade)),
                    "11" | "J" | "j" => Ok(Card(Jack, Spade)),
                    "12" | "Q" | "q" => Ok(Card(Queen, Spade)),
                    "13" | "K" | "k" => Ok(Card(King, Spade)),
                    _ => Err(()),
                },
                _ => Err(()),
            }
        }
    }

    pub fn parse(input: &str) -> Result<FullHand, ()> {
        let re = Regex::new(REGEX).unwrap();

        if !re.is_match(input) {
            return Err(());
        }

        let mut cards: Vec<Card> = Vec::with_capacity(5);

        for (_, [_, rank, suit]) in re.captures_iter(input).map(|c| c.extract()) {
            let Ok(card) = (rank, suit).try_into() else {
                return Err(());
            };
            cards.push(card);
        }

        if cards.len() != 5
            || (cards[0] == cards[1]
                || cards[0] == cards[2]
                || cards[0] == cards[3]
                || cards[0] == cards[4]
                || cards[1] == cards[2]
                || cards[1] == cards[3]
                || cards[1] == cards[4]
                || cards[2] == cards[3]
                || cards[2] == cards[4]
                || cards[3] == cards[4])
        {
            Err(())
        } else {
            Ok(FullHand(
                cards[0].clone(),
                cards[1].clone(),
                cards[2].clone(),
                cards[3].clone(),
                cards[4].clone(),
            ))
        }
    }

    #[test]
    fn test_parse_hand() {
        assert_eq!(
            parse("1d2d3d4d5d"),
            Ok(FullHand(
                Card(Ace, Diamond),
                Card(Two, Diamond),
                Card(Three, Diamond),
                Card(Four, Diamond),
                Card(Five, Diamond),
            ))
        );

        assert_eq!(
            parse("AD2D 3d.4d,5d"),
            Ok(FullHand(
                Card(Ace, Diamond),
                Card(Two, Diamond),
                Card(Three, Diamond),
                Card(Four, Diamond),
                Card(Five, Diamond),
            ))
        );

        assert_eq!(
            parse("10d11d12d13d1d"),
            Ok(FullHand(
                Card(Ten, Diamond),
                Card(Jack, Diamond),
                Card(Queen, Diamond),
                Card(King, Diamond),
                Card(Ace, Diamond),
            ))
        );

        assert_eq!(parse("1d2d3d4d"), Err(()));

        assert_eq!(parse("14d2d3d4d5d"), Err(()));

        assert_eq!(parse("1d1d2s3s4d"), Err(()));
    }
}
