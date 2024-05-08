use serde_derive::Deserialize;
use std::io;

// csvとは順不同
#[derive(Debug, Deserialize)]
// #[serde(rename_all = "PascalCase")] // すべて大文字ならこれでいい
struct Record {
    #[serde(rename = "Latitude")]
    latitude: f64,
    #[serde(rename = "Longitude")]
    longitude: f64,
    #[serde(rename = "Population")]
    #[serde(deserialize_with = "csv::invalid_option")]
    // どんなdeserialize error もNone valueに置き換える
    population: Option<u64>,
    city: String, // cityだけ小文字のケース
    #[serde(rename = "State")]
    state: String,
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    // deseriazlie で iteratorを作成
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record)
    }
    Ok(())
}
