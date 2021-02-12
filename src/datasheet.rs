use std::error::Error;
use std::io::{self, Write};
use std::process::Command;

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

pub fn run_tabula(page_range: &str, pdf: &str) -> io::Result<()> {
    let output = Command::new("java")
            .args(&["-jar", "bin/tabula.jar", "-p", page_range, pdf])
            .output()
            .expect("Fail");

    //println!("status: {}", output.status);
    //io::stderr().write_all(&output.stderr).unwrap();
        
    return io::stdout().write_all(&output.stdout);
}

// pub fn clean_device_attrs(csv_data: io::Read) -> Result<(), Box<dyn Error>> {
//     let mut rdr = csv::Reader::from_reader(&csv_data);
//     for result in rdr.deserialize() {
//         let record: Record = result?;

//         // Full address, i.e: FFFF EEEE to FFFFEEEE
//         let mut addr = record.address;
//         addr.retain(|ch| !ch.is_whitespace());
//         dbg!(&addr);
//     }
//     Ok(())
// }