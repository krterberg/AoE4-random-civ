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

    let mut civs: Vec<&str> = vec!(
        "Rus",
        "Holy Roman Empire",
        "Chinese",
        "English",
        "Delhi Sultanate",
        "Mongols",
        "Abbasid Dynasty",
        "French"
    );

    let includes_args = matches.value_of("Include");

    let includes = match includes_args {
        Some(str) => str.to_uppercase(),
        None => "RHCEDMAF".to_owned(),
    };

    let excludes_arg = matches.value_of("Exclude");

    let excludes = match excludes_arg {
        Some(str) => str.to_uppercase(),
        None => "".to_owned(),
    };

    let selection: String = match excludes {
        excludes if excludes.len() == 0 => includes,
        excludes => includes.chars().filter(|c| excludes.chars().any(|e| *c!=e)).collect(),
    };

    civs = civs
    .into_iter()
    .filter(|civ| selection.chars().any(|c| civ.starts_with(c)))
    .collect();

    if civs.len() == 0 {panic!("No valid civs selected after exclusions")}

    println!("You have selected the following options:\n{:?}", civs);

    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(0..civs.len());

    println!("{}", civs[roll]);
}
