pub fn goalkeeper_calc(table: table_extract::Table) {

    let mut goalkeeper_top_3_tuple: ((&str, i32), (&str, i32), (&str, i32))= (("No one", 0),("No one", 0), ("No one", 0));

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
        "The top three keepers are:\n\n1. {} ({})\n2. {} ({})\n3. {} ({})\n",
        goalkeeper_top_3_tuple.0.0,
        goalkeeper_top_3_tuple.0.1,
        goalkeeper_top_3_tuple.1.0,
        goalkeeper_top_3_tuple.1.1,
        goalkeeper_top_3_tuple.2.0,
        goalkeeper_top_3_tuple.2.1
    )

}

