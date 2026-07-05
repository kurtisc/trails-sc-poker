use check::Rank::*;
use check::Suit::*;
use check::{parse_input, tree_check, Card, FullHand};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn sample_hand() -> FullHand {
    FullHand(
        Card(King, Heart),
        Card(Eight, Diamond),
        Card(Three, Diamond),
        Card(Four, Diamond),
        Card(Six, Diamond),
    )
}

fn bench_parse(c: &mut Criterion) {
    c.bench_function("parse_input::parse", |b| {
        b.iter(|| parse_input::parse(black_box("10D JD QD KD AD")))
    });
}

fn bench_check_by_swaps(c: &mut Criterion) {
    let full_hand = sample_hand();
    let deck = (&full_hand).into();
    let cards = [
        &full_hand.0,
        &full_hand.1,
        &full_hand.2,
        &full_hand.3,
        &full_hand.4,
    ];

    let mut group = c.benchmark_group("tree_check::check");
    for swaps in 1..=3 {
        let keep = &cards[..5 - swaps];
        group.bench_function(format!("{swaps}_swap(s)"), |b| {
            b.iter(|| tree_check::check(black_box(keep), black_box(&deck)))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_parse, bench_check_by_swaps);
criterion_main!(benches);
