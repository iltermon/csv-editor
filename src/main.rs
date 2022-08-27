use std::error::Error;
use std::process;

fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let rdr = csv::Reader::from_path("src/username.csv");
    for result in rdr.unwrap().records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here..
        let record = result;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
