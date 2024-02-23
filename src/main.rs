
// extern crate reqwest;
use reqwest;
// use reqwest::headers::*;
use std::error::Error;
use csv::{Reader, ReaderBuilder, Writer};
use serde::Deserialize;
use serde_json;
// use std::io;
// use std::process;


#[derive(Debug, Deserialize, PartialEq, Clone)]
struct Record {
    date: String,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: i64,
}

fn remove_duplicates(records: &mut Vec<Record>) {
    records.sort_by(|a, b| a.date.cmp(&b.date));
    records.dedup_by(|a, b| a.date == b.date);
}


fn example(body: &String, symbol: &str) -> Result<(), Box<dyn Error>> {

    let mut existing_records: Vec<Record> = Vec::new();
    let rdr = Reader::from_path(format!("prices/{}.csv", symbol));
    let new_records: Vec<Record> = serde_json::from_str(body)?;

    if let Ok(mut rdr) = rdr {
        for result in rdr.deserialize() {
            let record: Record = result?;
            existing_records.push(record);
        }
    } else {
        println!("It didn't exit before");
    }


    // let length = existing_records.len();
    // println!("The length of the records vector is: {}", length);
    existing_records.splice(0..0, new_records.clone());

    remove_duplicates(&mut existing_records);
    let mut wtr = Writer::from_path(format!("prices/{}.csv", symbol))?;


    wtr.write_record(&["date", "open", "high", "low", "close", "volume"])?;
    for record in existing_records {
        wtr.write_record(&[
            &record.date.to_string(),
            &record.open.to_string(),
            &record.low.to_string(),
            &record.high.to_string(),
            &record.close.to_string(),
            &record.volume.to_string(),
        ])?;
    }
    wtr.flush()?;
    Ok(())
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let mut symbols_rdr = ReaderBuilder::new()
    .has_headers(false)
    .from_path("symbols.csv").unwrap();

    let mut symbols = Vec::new();

    
    for i in symbols_rdr.records() {
        symbols.push(i);
    }

    let symbol = if let Some(result) = symbols.get(0) {
        if let Ok(record) = result {
            // println!("{:?}", record.iter().collect::<Vec<_>>());
            // let symbol = record.iter().collect::<Vec<_>>();
            // println!("{:?}", symbol);
            record.iter().collect::<Vec<_>>()
        } else {
            println!("An error occurred while reading the record");
            vec!["None"]
        }
    } else {
        println!("The symbols vector is empty");
        vec!["None"]
    };
    
    for i in symbol {
        println!("{}", i);
        let body = reqwest::get("https://financialmodelingprep.com/api/v3/historical-chart/1hour/".to_owned()+ i +"?from=2023-08-10&to=2023-08-10&apikey=1198bd7c1dea75dcbe7333d021678f40")
        .await?
        .text()
        .await?;
        let _ = example(&body, i);
    }


    Ok(())
}



// mod bank;
// use bank::BankMain;

// fn main() {
//     let mut my_struct = BankMain::new("Nikita", 0);
//     my_struct.print_all_info();
//     my_struct.print_money();

//     my_struct.add_money(100);
//     my_struct.print_money();
// }
