#![allow(dead_code)]

use position_calcs;
use text_io::read;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {

    println!(
        "Please choose from the following:\n
        1: Goalkeepers
        2: Central Defenders
        3: Wing Backs
        4: Full Backs
        5: Defensive Midfielders
        6: Central Midfielders
        7: Wide Midfielders
        8: Wingers
        9: Central Attacking Mids
        10: Strikers
        "
    );

    let user_choice: i32 = read!();

    match user_choice {
        1 => position_calcs::retrieve_important_attributes("Goalkeepers"),
        2 => position_calcs::retrieve_important_attributes("Central Defenders"),
        3 => position_calcs::retrieve_important_attributes("Wing Backs"),
        4 => position_calcs::retrieve_important_attributes("Full Backs"),
        5 => position_calcs::retrieve_important_attributes("Defensive Midfielders"),
        6 => position_calcs::retrieve_important_attributes("Central Midfielders"),
        7 => position_calcs::retrieve_important_attributes("Wide Midfielders"),
        8 => position_calcs::retrieve_important_attributes("Wingers"),
        9 => position_calcs::retrieve_important_attributes("Central Attacking Mids"),
        10 => position_calcs::retrieve_important_attributes("Strikers"),
        _ => println!("Not yet implemented.")
    }
}
