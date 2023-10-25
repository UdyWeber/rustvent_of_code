mod day_one;
mod day_two;

fn main() {
    println!("In day one the maximum calories brought by the elves was {}", day_one::day_one::day_one_resolution());
    println!("Game points: {}", day_two::day_two::day_two_resolution());
}
