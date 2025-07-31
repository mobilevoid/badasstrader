use csv::ReaderBuilder;
use serde::Deserialize;
use badasstrader::moving_average;

#[derive(Debug, Deserialize)]
struct Record {
    date: String,
    close: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <csv file>", args[0]);
        std::process::exit(1);
    }
    let path = &args[1];
    let mut rdr = ReaderBuilder::new().from_path(path)?;
    let mut closes = Vec::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        closes.push(record.close);
    }

    let short_ma = moving_average(&closes, 5);
    let long_ma = moving_average(&closes, 20);

    if short_ma.is_empty() || long_ma.is_empty() {
        println!("Not enough data to generate signal.");
        return Ok(());
    }

    let latest_short = short_ma.last().unwrap();
    let latest_long = long_ma.last().unwrap();
    if latest_short > latest_long {
        println!("Buy signal!");
    } else if latest_short < latest_long {
        println!("Sell signal!");
    } else {
        println!("Hold.");
    }
    Ok(())
}
