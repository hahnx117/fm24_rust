use std::fs;

const DATA_PATH: &str = "/home/david/rust_projects/fm24_rust/data/attributes.html";

pub fn retrieve_important_attributes (position_name: &str) {
    match position_name {
        "Goalkeepers" => position_calc(position_name, "Han", "Pos", "Ref", "1v1", "Aer"),
        _ => println!("Not Yet Implemented...")
    }
}

//pub fn position_calc(table: table_extract::Table) {
fn position_calc(position_name: &str, att1: &str, att2: &str, att3: &str, att4: &str, att5: &str) {

    let mut top_3_tuple: ((&str, i32), (&str, i32), (&str, i32))= (("No one", 0),("No one", 0), ("No one", 0));

    let data: String = fs::read_to_string(DATA_PATH).expect("Should have been able to read the file.");
    let table: table_extract::Table = table_extract::Table::find_first(&data).unwrap();

    for row in &table {
        let name: &str = row.get("Name").unwrap_or("No name");
        let stat1: i32 = row.get(att1).unwrap_or("0").parse().unwrap();
        let stat2: i32 = row.get(att2).unwrap_or("0").parse().unwrap();
        let stat3: i32 = row.get(att3).unwrap_or("0").parse().unwrap();
        let stat4: i32 = row.get(att4).unwrap_or("0").parse().unwrap();
        let stat5: i32 = row.get(att5).unwrap_or("0").parse().unwrap();
        
        let total_stats: i32 = stat1 + stat2 + stat3 + stat4 + stat5;
        
        if total_stats > top_3_tuple.0.1 {
            top_3_tuple.0.0 = name;
            top_3_tuple.0.1 = total_stats;
        }
        else if total_stats > top_3_tuple.1.1 {
            top_3_tuple.1.0 = name;
            top_3_tuple.1.1 = total_stats;
        }
        else if total_stats > top_3_tuple.2.1 {
            top_3_tuple.2.0 = name;
            top_3_tuple.2.1 = total_stats;
        }
    }

    println!(
        "The top three {position_name} are:\n\n1. {} ({})\n2. {} ({})\n3. {} ({})\n",
        top_3_tuple.0.0,
        top_3_tuple.0.1,
        top_3_tuple.1.0,
        top_3_tuple.1.1,
        top_3_tuple.2.0,
        top_3_tuple.2.1
    )

}