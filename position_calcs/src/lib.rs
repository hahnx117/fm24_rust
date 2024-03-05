pub fn goalkeeper_calc(table: table_extract::Table) {

    let mut top_3_tuple: ((&str, i32), (&str, i32), (&str, i32))= (("No one", 0),("No one", 0), ("No one", 0));

    for row in &table {
        let name: &str = row.get("Name").unwrap_or("No name");
        let handling: i32 = row.get("Han").unwrap_or("0").parse().unwrap();
        let positioning: i32 = row.get("Pos").unwrap_or("0").parse().unwrap();
        let reflexes: i32 = row.get("Ref").unwrap_or("0").parse().unwrap();
        let one_on_ones: i32 = row.get("1v1").unwrap_or("0").parse().unwrap();
        let aerial_ability: i32 = row.get("Aer").unwrap_or("0").parse().unwrap();
        
        let total_goalkeeping: i32 = handling + positioning + reflexes + one_on_ones + aerial_ability;
        
        if total_goalkeeping > top_3_tuple.0.1 {
            top_3_tuple.0.0 = name;
            top_3_tuple.0.1 = total_goalkeeping;
        }
        else if total_goalkeeping > top_3_tuple.1.1 {
            top_3_tuple.1.0 = name;
            top_3_tuple.1.1 = total_goalkeeping;
        }
        else if total_goalkeeping > top_3_tuple.2.1 {
            top_3_tuple.2.0 = name;
            top_3_tuple.2.1 = total_goalkeeping;
        }
    }

    println!(
        "The top three keepers are:\n\n1. {} ({})\n2. {} ({})\n3. {} ({})\n",
        top_3_tuple.0.0,
        top_3_tuple.0.1,
        top_3_tuple.1.0,
        top_3_tuple.1.1,
        top_3_tuple.2.0,
        top_3_tuple.2.1
    )

}

pub fn central_defender_calc(table: table_extract::Table) {

    let mut top_3_tuple: ((&str, i32), (&str, i32), (&str, i32))= (("No one", 0),("No one", 0), ("No one", 0));

    for row in &table {
        let name: &str = row.get("Name").unwrap_or("No name");
        let tackling: i32 = row.get("Tck").unwrap_or("0").parse().unwrap();
        let jumping: i32 = row.get("Jum").unwrap_or("0").parse().unwrap();
        let strength: i32 = row.get("Str").unwrap_or("0").parse().unwrap();
        let marking: i32 = row.get("Mar").unwrap_or("0").parse().unwrap();
        let concentration: i32 = row.get("Cnt").unwrap_or("0").parse().unwrap();
        
        let total_defending: i32 = tackling + jumping + strength + marking + concentration;
        
        if total_defending > top_3_tuple.0.1 {
            top_3_tuple.0.0 = name;
            top_3_tuple.0.1 = total_defending;
        }
        else if total_defending > top_3_tuple.1.1 {
            top_3_tuple.1.0 = name;
            top_3_tuple.1.1 = total_defending;
        }
        else if total_defending > top_3_tuple.2.1 {
            top_3_tuple.2.0 = name;
            top_3_tuple.2.1 = total_defending;
        }
    }

    println!(
        "The top three central defenders are:\n\n1. {} ({})\n2. {} ({})\n3. {} ({})\n",
        top_3_tuple.0.0,
        top_3_tuple.0.1,
        top_3_tuple.1.0,
        top_3_tuple.1.1,
        top_3_tuple.2.0,
        top_3_tuple.2.1
    )

}