#![allow(dead_code)]
use std::fs;

fn goalkeeper_calc(table: table_extract::Table) {

    let mut goalkeeper_top_3_tuple= (("No one", 0),("No one", 0), ("No one", 0));

    for row in &table {
        let name: &str = row.get("Name").unwrap_or("No name");
        let handling: i32 = row.get("Han").unwrap_or("0").parse().unwrap();
        let positioning: i32 = row.get("Pos").unwrap_or("0").parse().unwrap();
        let reflexes: i32 = row.get("Ref").unwrap_or("0").parse().unwrap();
        let one_on_ones: i32 = row.get("1v1").unwrap_or("0").parse().unwrap();
        let aerial_ability: i32 = row.get("Aer").unwrap_or("0").parse().unwrap();
        
        let total_goalkeeping: i32 = handling + positioning + reflexes + one_on_ones + aerial_ability;
        
        if total_goalkeeping > goalkeeper_top_3_tuple.0.1 {
            goalkeeper_top_3_tuple.0.0 = name;
            goalkeeper_top_3_tuple.0.1 = total_goalkeeping;
        }
        else if total_goalkeeping > goalkeeper_top_3_tuple.1.1 {
            goalkeeper_top_3_tuple.1.0 = name;
            goalkeeper_top_3_tuple.1.1 = total_goalkeeping;
        }
        else if total_goalkeeping > goalkeeper_top_3_tuple.2.1 {
            goalkeeper_top_3_tuple.2.0 = name;
            goalkeeper_top_3_tuple.2.1 = total_goalkeeping;
        }
    }

    println!(
        "The top three keepers are:\n {}\n{}\n{}\n with scores of {}, {}, and {}, respectively.",
        goalkeeper_top_3_tuple.0.0,
        goalkeeper_top_3_tuple.1.0,
        goalkeeper_top_3_tuple.2.0,
        goalkeeper_top_3_tuple.0.1,
        goalkeeper_top_3_tuple.1.1,
        goalkeeper_top_3_tuple.2.1
    )
} 

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
