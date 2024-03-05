#![allow(dead_code)]

use std::fs;
use position_calcs;
use text_io::read;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let data_path = "/home/david/rust_projects/fm24_rust/data/attributes.html";

    let data = fs::read_to_string(data_path).expect("Should have been able to read the file.");
    //println!("{}", data);

    let table = table_extract::Table::find_first(&data).unwrap();

    println!(
        "Please choose from the following:\n
        1: Goalkeepers
        2: Defenders
        3. Midfielders
        4. Strikers"
    );

    let user_choice: i32 = read!();

    match user_choice {
        1 => position_calcs::goalkeeper_calc(table),
        _ => println!("Not yet implemented.")
    }
}
