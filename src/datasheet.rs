use std::error::Error;
use std::process::{Command, Output};

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

/// Runs tabula PDF OCR
/// tabula.jar:
// wget https://github.com/tabulapdf/tabula-java/releases/download/v1.0.4/tabula-1.0.4-jar-with-dependencies.jar

pub fn run_tabula(datasheet: &str, page_range: &str) -> Output {
    let output = Command::new("java")
            .args(&["-jar", "bin/tabula.jar", "-p", page_range, datasheet])
            .output()
            .expect("Fail");

    // TODO: Error ctrl
    //println!("status: {}", output.status);
    //io::stderr().write_all(&output.stderr).unwrap();
        
    return output;
}

pub fn clean_device_attrs(csv_data: Output) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(&*csv_data.stdout);
    for result in rdr.deserialize() {
        let record: Record = result?;

        // Full address, i.e: FFFF EEEE to FFFFEEEE
        let mut addr = record.address;
        let name = record.name;

        addr.retain(|ch| !ch.is_whitespace());

        if &addr != "" && &addr != "Address" {
            dbg!(&addr);
        }

        if &name != "" && &name != "Function Register Name" {
            dbg!(&name);
        }
    }
    Ok(())
}