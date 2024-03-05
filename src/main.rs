#![allow(dead_code)]

use std::fs;
use position_calcs;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let data_path = "/home/david/rust_projects/fm24_rust/data/attributes.html";

    let data = fs::read_to_string(data_path).expect("Should have been able to read the file.");
    //println!("{}", data);

    let table = table_extract::Table::find_first(&data).unwrap();
    //let table = table_extract::Table::find_by_headers(&data, &["data"]).unwrap();

    //println!("{:#?}", table);

    //for row in &table {
    //    println!(
    //        "{} is worth {} and is on {} wages",
    //        row.get("Name").unwrap_or("<name missing>"),
    //        row.get("Transfer Value").unwrap_or("<transfer value missing>"),
    //        row.get("Wage").unwrap_or("<wage value missing>")
    //    )
    //}
    goalkeeper_calc(table);


}
