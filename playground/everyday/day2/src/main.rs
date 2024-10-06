mod clock;
use serde_derive::Deserialize;
use std::{env, io};

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
    // 第一引数を受け取る
    let query = match env::args().nth(1) {
        Some(query) => query,
        None => return Err(From::from("expected 1 arg, but got none")),
    };

    // csv reader と writerを作成
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wdr = csv::Writer::from_writer(io::stderr());

    // header情報だけは先に書き込んでおく
    wdr.write_record(rdr.headers()?)?;

    for result in rdr.records() {
        let record = result?;
        // println!("{:?}", record);
        if record.iter().any(|field| field == &query) {
            // println!("{:?}", &record)
            wdr.write_record(&record)?;
        }
    }
    // internal bufferを使うので、終了時にflush
    wdr.flush()?;
    Ok(())
}

// fn run() -> Result<(), Box<dyn std::error::Error>> {
//     let mut rdr = csv::Reader::from_reader(io::stdin());
//     // deseriazlie で iteratorを作成
//     for result in rdr.deserialize() {
//         let record: Record = result?;
//         println!("{:?}", record)
//     }
//     Ok(())
// }
