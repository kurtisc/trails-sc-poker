#[cfg(feature="big-tests")]
use crate::*;

#[cfg(all(test, feature = "big-tests"))]
mod tests {
    use super::*;
    use crate::parse_input::*;

    macro_rules! straights {
        () => {
            [
                straights!(@1 1),
                straights!(@1 2),
                straights!(@1 3),
                straights!(@1 4),
                straights!(@1 5),
                straights!(@1 6),
                straights!(@1 7),
                straights!(@1 8),
                straights!(@1 9),
                straights!(@1 10),
            ].concat()
        };

        (@1 $rank: literal) => {
            [
                straights!(@4 $rank, "c", "h", "s", "d"),
            ].concat()
        };

        (@2 $rank: literal, $suit1: literal, $suit2: literal, $suit3: literal, $suit4: literal) => {
                [
                    straights!(@3 $rank, $suit1, $suit2, $suit3, $suit4),
                    straights!(@3 $rank, $suit2, $suit2, $suit3, $suit4),
                    straights!(@3 $rank, $suit3, $suit2, $suit3, $suit4),
                    straights!(@3 $rank, $suit4, $suit2, $suit3, $suit4),
                ].concat()
        };

        (@3 $rank: literal, $suit1: literal, $suit2: literal, $suit3: literal, $suit4: literal) => {
                [
                    straights!(@4 $rank, $suit1, $suit1, $suit3, $suit4),
                    straights!(@4 $rank, $suit1, $suit2, $suit3, $suit4),
                    straights!(@4 $rank, $suit1, $suit3, $suit3, $suit4),
                    straights!(@4 $rank, $suit1, $suit4, $suit3, $suit4),
                ].concat()
        };

        (@4 $rank: literal, $suit1: literal, $suit2: literal, $suit3: literal, $suit4: literal) => {
                [
                    straights!(@5 $rank, $suit1, $suit2, $suit1, $suit4),
                    straights!(@5 $rank, $suit1, $suit2, $suit2, $suit4),
                    straights!(@5 $rank, $suit1, $suit2, $suit3, $suit4),
                    straights!(@5 $rank, $suit1, $suit2, $suit4, $suit4),
                ].concat()
        };

        (@5 $rank: literal, $suit1: literal, $suit2: literal, $suit3: literal, $suit4: literal) => {
                [
                    straights!(@6 $rank, $suit1, $suit2, $suit3, $suit1),
                    straights!(@6 $rank, $suit1, $suit2, $suit3, $suit2),
                    straights!(@6 $rank, $suit1, $suit2, $suit3, $suit3),
                    straights!(@6 $rank, $suit1, $suit2, $suit3, $suit4),
                ].concat()
        };

        (@6 $rank: literal, $suit1: literal, $suit2: literal, $suit3: literal, $suit4: literal) => {
            [
                straights!(@check $rank, $suit1, $suit2, $suit3, $suit4, $suit1),
                straights!(@check $rank, $suit1, $suit2, $suit3, $suit4, $suit2),
                straights!(@check $rank, $suit1, $suit2, $suit3, $suit4, $suit3),
                straights!(@check $rank, $suit1, $suit2, $suit3, $suit4, $suit4),
            ]
        };

        (@check $rank: literal, $suit1: literal, $suit2: literal, $suit3: literal, $suit4: literal, $suit5: literal) => {
            if $suit1 != $suit2
                && $suit1 != $suit3
                && $suit1 != $suit4
                && $suit1 != $suit5
                && $suit2 != $suit3
                && $suit2 != $suit4
                && $suit2 != $suit5
                && $suit3 != $suit4
                && $suit3 != $suit5
                && $suit4 != $suit5
            {
                straights!(@parse $rank, $suit1, $suit2, $suit3, $suit4, $suit5)
            } else {
                Err(())
            }
        };

        (@parse $rank: literal, $suit1: literal, $suit2: literal, $suit3: literal, $suit4: literal, $suit5: literal) => {{
            parse(&format!("{}{} {}{} {}{} {}{} {}{}",
                    $rank, $suit1,
                    $rank + 1, $suit2,
                    $rank + 2, $suit3,
                    $rank + 3, $suit4,
                    (($rank + 4 - 1) % 13 + 1), $suit5))
        }}
    }

    #[test]
    fn test_pair() {
        let full_hand = FullHand(
            Card(Ace, Club),
            Card(Ace, Heart),
            Card(Two, Heart),
            Card(Three, Heart),
            Card(Four, Heart),
        );

        let hand = [
            &full_hand.0,
            &full_hand.1,
            &full_hand.2,
            &full_hand.3,
            &full_hand.4,
        ];

        assert_eq!(check::check(&hand), Some(Score::Pair));
    }

    #[test]
    fn test_two_pair() {
        let full_hand = FullHand(
            Card(Ace, Club),
            Card(Ace, Heart),
            Card(Three, Spade),
            Card(Three, Heart),
            Card(Four, Heart),
        );

        let hand = [
            &full_hand.0,
            &full_hand.1,
            &full_hand.2,
            &full_hand.3,
            &full_hand.4,
        ];

        assert_eq!(check::check(&hand), Some(Score::TwoPair));
    }

    #[test]
    fn test_three_of_a_kind() {
        let full_hand = FullHand(
            Card(Ace, Club),
            Card(Ace, Heart),
            Card(Ace, Spade),
            Card(Three, Heart),
            Card(Four, Heart),
        );

        let hand = [
            &full_hand.0,
            &full_hand.1,
            &full_hand.2,
            &full_hand.3,
            &full_hand.4,
        ];

        assert_eq!(check::check(&hand), Some(Score::ThreeOfAKind));
    }

    #[test]
    fn test_straight() {
        let hands = straights!();

        for full_hand in hands {
            if full_hand.is_ok() {
                let full_hand = full_hand.unwrap();
                let hand = [
                    &full_hand.0,
                    &full_hand.1,
                    &full_hand.2,
                    &full_hand.3,
                    &full_hand.4,
                ];

                assert_eq!((check::check(&hand), &hand), (Some(Score::Straight), &hand));
            }
        }
    }

    #[test]
    fn test_flush() {
        let hands = [
            parse("1c  2c  3c  4c  6c").unwrap(),
            parse("2c  3c  4c  5c  7c").unwrap(),
            parse("3c  4c  5c  6c  8c").unwrap(),
            parse("4c  5c  6c  7c  9c").unwrap(),
            parse("5c  6c  7c  8c  10c").unwrap(),
            parse("6c  7c  8c  9c  11c").unwrap(),
            parse("7c  8c  9c  10c 12c").unwrap(),
            parse("8c  9c  10c 11c 13c").unwrap(),
            parse("9c  10c 11c 12c  1c").unwrap(),
            parse("10c 11c 12c 13c  2c").unwrap(),
            parse("11c 12c 13c  1c  3c").unwrap(),
            parse("12c 13c 1c   2c  4c").unwrap(),
            parse("13c  1c 2c   3c  5c").unwrap(),
            parse("1d  2d  3d  4d  6d").unwrap(),
            parse("2d  3d  4d  5d  7d").unwrap(),
            parse("3d  4d  5d  6d  8d").unwrap(),
            parse("4d  5d  6d  7d  9d").unwrap(),
            parse("5d  6d  7d  8d  10d").unwrap(),
            parse("6d  7d  8d  9d  11d").unwrap(),
            parse("7d  8d  9d  10d 12d").unwrap(),
            parse("8d  9d  10d 11d 13d").unwrap(),
            parse("9d  10d 11d 12d  1d").unwrap(),
            parse("10d 11d 12d 13d  2d").unwrap(),
            parse("11d 12d 13d  1d  3d").unwrap(),
            parse("12d 13d 1d   2d  4d").unwrap(),
            parse("13d  1d 2d   3d  5d").unwrap(),
            parse("1h  2h  3h  4h  6h").unwrap(),
            parse("2h  3h  4h  5h  7h").unwrap(),
            parse("3h  4h  5h  6h  8h").unwrap(),
            parse("4h  5h  6h  7h  9h").unwrap(),
            parse("5h  6h  7h  8h  10h").unwrap(),
            parse("6h  7h  8h  9h  11h").unwrap(),
            parse("7h  8h  9h  10h 12h").unwrap(),
            parse("8h  9h  10h 11h 13h").unwrap(),
            parse("9h  10h 11h 12h  1h").unwrap(),
            parse("10h 11h 12h 13h  2h").unwrap(),
            parse("11h 12h 13h  1h  3h").unwrap(),
            parse("12h 13h 1h   2h  4h").unwrap(),
            parse("13h  1h 2h   3h  5h").unwrap(),
            parse("1s  2s  3s  4s  6s").unwrap(),
            parse("2s  3s  4s  5s  7s").unwrap(),
            parse("3s  4s  5s  6s  8s").unwrap(),
            parse("4s  5s  6s  7s  9s").unwrap(),
            parse("5s  6s  7s  8s  10s").unwrap(),
            parse("6s  7s  8s  9s  11s").unwrap(),
            parse("7s  8s  9s  10s 12s").unwrap(),
            parse("8s  9s  10s 11s 13s").unwrap(),
            parse("9s  10s 11s 12s  1s").unwrap(),
            parse("10s 11s 12s 13s  2s").unwrap(),
            parse("11s 12s 13s  1s  3s").unwrap(),
            parse("12s 13s 1s   2s  4s").unwrap(),
            parse("13s  1s 2s   3s  5s").unwrap(),
        ];

        for full_hand in hands {
            let hand = [
                &full_hand.0,
                &full_hand.1,
                &full_hand.2,
                &full_hand.3,
                &full_hand.4,
            ];

            assert_eq!(check::check(&hand), Some(Score::Flush));
        }
    }

    #[test]
    fn test_full_house() {
        let full_hand = FullHand(
            Card(Ace, Club),
            Card(Ace, Heart),
            Card(Ace, Spade),
            Card(Three, Heart),
            Card(Three, Spade),
        );

        let hand = [
            &full_hand.0,
            &full_hand.1,
            &full_hand.2,
            &full_hand.3,
            &full_hand.4,
        ];

        assert_eq!(check::check(&hand), Some(Score::FullHouse));
    }

    #[test]
    fn test_four_of_a_kind() {
        let full_hand = FullHand(
            Card(Ace, Club),
            Card(Ace, Heart),
            Card(Ace, Spade),
            Card(Ace, Diamond),
            Card(Four, Heart),
        );

        let hand = [
            &full_hand.0,
            &full_hand.1,
            &full_hand.2,
            &full_hand.3,
            &full_hand.4,
        ];

        assert_eq!(check::check(&hand), Some(Score::FourOfAKind));
    }

    #[test]
    fn test_straight_flush() {
        macro_rules! straight_flushes {
            () => {
                [
                    straight_flushes!(1),
                    straight_flushes!(2),
                    straight_flushes!(3),
                    straight_flushes!(4),
                    straight_flushes!(5),
                    straight_flushes!(6),
                    straight_flushes!(7),
                    straight_flushes!(8),
                    straight_flushes!(9),
                ].concat()
            };

            ($rank: literal) => {
                [
                    straights!(@parse $rank, "c", "c", "c", "c", "c"),
                    straights!(@parse $rank, "h", "h", "h", "h", "h"),
                    straights!(@parse $rank, "s", "s", "s", "s", "s"),
                    straights!(@parse $rank, "d", "d", "d", "d", "d")
                ]
            };
        }

        let hands = straight_flushes!();

        for full_hand in hands {
            if full_hand.is_ok() {
                let full_hand = full_hand.unwrap();
                let hand = [
                    &full_hand.0,
                    &full_hand.1,
                    &full_hand.2,
                    &full_hand.3,
                    &full_hand.4,
                ];

                assert_eq!(check::check(&hand), Some(Score::StraightFlush));
            }
        }
    }

    #[test]
    fn test_royal_flush() {
        let hands = [
            parse("10c  11c 12c 13c 1c").unwrap(),
            parse("10d  11d 12d 13d 1d").unwrap(),
            parse("10h  11h 12h 13h 1h").unwrap(),
            parse("10s  11s 12s 13s 1s").unwrap(),
        ];

        for full_hand in hands {
            let hand = [
                &full_hand.0,
                &full_hand.1,
                &full_hand.2,
                &full_hand.3,
                &full_hand.4,
            ];

            assert_eq!(check::check(&hand), Some(Score::RoyalFlush));
        }
    }
}
