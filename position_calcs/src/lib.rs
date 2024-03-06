use std::fs;

const DATA_PATH: &str = "/home/david/rust_projects/fm24_rust/data/attributes.html";

pub fn retrieve_important_attributes (position_name: &str) {
    match position_name {
        "Goalkeepers" => position_calc(position_name, "Han", "Pos", "Ref", "1v1", "Aer"),
        "Central Defenders" => position_calc(position_name, "Tck", "Jum", "Str", "Mar", "Cnt"),
        "Wing Backs" => position_calc(position_name, "Cro", "Tea", "Pac", "Mar", "Dec"),
        "Full Backs" => position_calc(position_name, "Tck", "Tea", "Pac", "Mar", "Dec"),
        "Defensive Midfielders" => position_calc(position_name, "Tck", "Pas", "Bra", "Det", "Wor"),
        "Central Midfielders" => position_calc(position_name, "Tck", "Pos", "Tea", "Dec", "Det"),
        "Wide Midfielders" => position_calc(position_name, "Pas", "Cro", "Dri", "Pac", "Tea"),
        "Wingers" => position_calc(position_name, "Pas", "Cro", "Dri", "Pac", "Tea"),
        "Central Attacking Mids" => position_calc(position_name, "Pas", "Dec", "Fla", "Tec", "Fir"),
        "Strikers" => position_calc(position_name, "Fin", "Cmp", "Pac", "Acc", "Dri"),
        _ => println!("Not Yet Implemented...")
    }
}

fn hyphen_filter(stat_string: &str) -> i32 {
    if stat_string.contains("-") {
        let split_stat = stat_string.split("-").collect::<Vec<_>>();
        return split_stat[0].parse().unwrap();
    }
    else {
        return stat_string.parse().unwrap()
    }
}

fn position_calc(position_name: &str, att1: &str, att2: &str, att3: &str, att4: &str, att5: &str) {

    let mut top_3_tuple: (
        (&str, i32, &str, &str, &str, &str, &str, &str),
        (&str, i32, &str, &str, &str, &str, &str, &str),
        (&str, i32, &str, &str, &str, &str, &str, &str),
    ) = (
        ("", 0, "", "", "", "", "", ""),
        ("", 0, "", "", "", "", "", ""),
        ("", 0, "", "", "", "", "", "")
    );

    let data: String = fs::read_to_string(DATA_PATH).expect("Should have been able to read the file.");
    let table: table_extract::Table = table_extract::Table::find_first(&data).unwrap();

    for row in &table {
        let name: &str = row.get("Name").unwrap_or("No name");
        let stat1: i32 = hyphen_filter(row.get(att1).unwrap_or("0"));
        let stat2: i32 = hyphen_filter(row.get(att2).unwrap_or("0"));
        let stat3: i32 = hyphen_filter(row.get(att3).unwrap_or("0"));
        let stat4: i32 = hyphen_filter(row.get(att4).unwrap_or("0"));
        let stat5: i32 = hyphen_filter(row.get(att5).unwrap_or("0"));
        let wage: &str = row.get("Wage").unwrap_or("0");
        let media_description: &str = row.get("Media Description").unwrap_or("0");
        let media_handling: &str = row.get("Media Handling").unwrap_or("0");
        let nationality: &str = row.get("Nat").unwrap_or("0");
        let personality: &str = row.get("Personality").unwrap_or("0");
        let position: &str = row.get("Position").unwrap_or("0");
        
        let total_stats: i32 = stat1 + stat2 + stat3 + stat4 + stat5;
        
        if total_stats > top_3_tuple.0.1 {
            top_3_tuple.0.0 = name;
            top_3_tuple.0.1 = total_stats;
            top_3_tuple.0.2 = wage;
            top_3_tuple.0.3 = media_description;
            top_3_tuple.0.4 = media_handling;
            top_3_tuple.0.5 = nationality;
            top_3_tuple.0.6 = personality;
            top_3_tuple.0.7 = position;
        }
        else if total_stats > top_3_tuple.1.1 {
            top_3_tuple.1.0 = name;
            top_3_tuple.1.1 = total_stats;
            top_3_tuple.1.2 = wage;
            top_3_tuple.1.3 = media_description;
            top_3_tuple.1.4 = media_handling;
            top_3_tuple.1.5 = nationality;
            top_3_tuple.1.6 = personality;
            top_3_tuple.1.7 = position;
        }
        else if total_stats > top_3_tuple.2.1 {
            top_3_tuple.2.0 = name;
            top_3_tuple.2.1 = total_stats;
            top_3_tuple.2.2 = wage;
            top_3_tuple.2.3 = media_description;
            top_3_tuple.2.4 = media_handling;
            top_3_tuple.2.5 = nationality;
            top_3_tuple.2.6 = personality;
            top_3_tuple.2.7 = position;
        }
    }

    println!("The top three {} are:\n", position_name);

    for player in [top_3_tuple.0, top_3_tuple.1, top_3_tuple.2] {
        println!("Name: {}\nPosition Score: {}\nNatural Positions: {}\nWages: {}\nMedia Description: {}\nMedia Handling: {}\nNationality: {}\nPersonality: {}\n\n",
        player.0, player.1, player.7, player.2, player.3, player.4, player.5, player.6
        )
    }

}