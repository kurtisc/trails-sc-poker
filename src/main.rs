use check::parse_input;
use check::tree_check;

fn main() {
    use std::io::stdin;

    print_usage();
    let mut hand_string = String::new();
    let mut multiplier_string = String::new();

    loop {
        hand_string.clear();
        multiplier_string.clear();

        println!("Enter hand:");
        stdin()
            .read_line(&mut hand_string)
            .expect("Did not enter a correct hand");

        let Ok(full_hand) = parse_input::parse(&hand_string) else {
            print_usage();
            continue;
        };

        println!("Enter current multiplier:");
        stdin()
            .read_line(&mut multiplier_string)
            .expect("Did not enter a correct multiplier");

        let multiplier: i32 = multiplier_string.trim().parse().expect("Input not an integer");

        let deck = (&full_hand).into();
        tree_check::best_score(full_hand, &deck, Some(multiplier));
    }
}

fn print_usage() {
    println!("Type the hand you've been dealt");
    println!("Accepted formats:");
    println!("10D JD QD KD AD");
    println!("10D,JD,QD,KD,AD");
    println!("10D.JD,QD kD,1D");
    println!("10d11d12d13d1d");
    println!("etc!");
}
