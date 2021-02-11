use std::error::Error;
use std::io;
use std::process;

use csv;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    #[serde(alias = "Address")]
    address: String,
    #[serde(alias = "Function Register Name")]
    name: String,
    #[serde(alias = "R/W")]
    mode: String,
    #[serde(alias = "Bit Units\rfor Manipulation")]
    manip: String,
    #[serde(alias = "Initial\rValue")]
    initial: String,
}

fn clean_device_attrs() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Record = result?;

        // Full address, i.e: FFFF EEEE to FFFFEEEE
        let mut addr = record.address;
        addr.retain(|ch| !ch.is_whitespace());
        dbg!(&addr);
    }
    Ok(())
}