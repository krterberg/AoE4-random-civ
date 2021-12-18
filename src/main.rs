use rand::Rng;
use clap::{Arg, App};
fn main() {
    
    let matches = App::new("Random civ roller")
    .version("0.2.0")
    .about("Picks a random civ from the game Age of Empires 4 for you")
    .arg(Arg::with_name("Include")
             .short("i")
             .long("include")
             .takes_value(true)
             .required(false)
             .help("First letters of civs to include, as a string"))
    .arg(Arg::with_name("Exclude")
             .short("e")
             .long("exclude")
             .takes_value(true)
             .required(false)
             .help("First letters of civs to exclude, as a string"))
    .get_matches();

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
