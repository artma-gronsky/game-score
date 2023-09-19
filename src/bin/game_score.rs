use game_score::{generate_game, get_score};

fn main() {
    let stamps = generate_game();

    let (home, away) = get_score(&stamps, 10000000);

    println!("Home = {}, Away = {}", home, away);
}
