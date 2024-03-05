use htmler::Html;
use std::fs;


fn main() {
    let data_path = "/home/david/Documents/fm24_rust/data/attributes.html";

    let data = fs::read_to_string(data_path).expect("Should have been able to read the file.");
    println!("{}", data);

    //let document = Html::parse_document(&data);

    //println!("{:?}", document)

    let table = table_extract::Table::find_first(&data);

    println!("{:#?}", table);

}
