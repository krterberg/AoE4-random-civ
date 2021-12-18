use rand::Rng;
use clap::{Arg, App};
fn main() {
    

    let civs: Vec<&str> = vec!(
        "Rus",
        "Holy Roman Empire",
        "Chinese",
        "English",
        "Delhi Sultanate",
        "Mongols",
        "Abbasid Dynasty",
        "French"
    );

    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(0..civs.len());

    println!("{}", civs[roll]);
}
