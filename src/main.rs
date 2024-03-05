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
        "
    );

    let user_choice: i32 = read!();

    match user_choice {
        1 => position_calcs::retrieve_important_attributes("Goalkeepers"),
        _ => println!("Not yet implemented.")
    }
}
