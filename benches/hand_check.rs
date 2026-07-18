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
    for swaps in 1..=5 {
        let keep = &cards[..5 - swaps];
        group.bench_function(format!("{swaps}_swap(s)"), |b| {
            b.iter(|| tree_check::check(black_box(keep), black_box(&deck)))
        });
    }
    group.finish();
}

// The realistic end-to-end workload: ranked_swap_values checks all 32
// subsets of the hand (the powerset), so it pays for the worst (5-swap)
// case eight times over. This is what a single interactive query costs.
fn bench_ranked_swap_values(c: &mut Criterion) {
    let full_hand = sample_hand();
    let deck = (&full_hand).into();

    c.bench_function("tree_check::ranked_swap_values", |b| {
        b.iter(|| tree_check::ranked_swap_values(black_box(&full_hand), black_box(&deck), 1))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(20);
    targets = bench_parse, bench_check_by_swaps, bench_ranked_swap_values
}
criterion_main!(benches);
