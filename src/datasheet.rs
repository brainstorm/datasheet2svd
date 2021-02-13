use std::error::Error;
use std::process::{Command, Output};

use csv::{Reader, StringRecord};
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    address: String,
    description: String,
    name: String,
    mode: String,
    manip_1_bit: String,
    manip_8_bit: String,
    manip_16_bit: String,
    reset_value: String,
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
    let mut rdr = Reader::from_reader(&*csv_data.stdout);
    rdr.set_headers(StringRecord::from(vec!["address", "description", "name", "mode", "manip_1_bit", "manip_8_bit", "manip_16_bit", "reset_value"]));

    for result in rdr.deserialize() {
        let record: Record = result?;

        let mut addr = record.address;
        let descr = record.description;
        let name = record.name;
        let mut mode = record.mode;
        let reset_value = record.reset_value;

        // Skip repeated headers over pages
        if &addr == "Address" {
            continue;
        }

        // Handle multi-row descriptions for registers
        if &addr == "" {
            if &name == "" { continue; } // beginning?
            dbg!("WARNING: Multi-row register definition");
            dbg!(&name);
            dbg!(&mode);
            dbg!(&reset_value);
            continue;
        }

        // Full address, i.e: FFFF EEEE to FFFFEEEE
        addr.retain(|ch| !ch.is_whitespace());

        dbg!(&addr);
        dbg!(&name); // TODO: Disambiguate cases like "SIRB2/ SIRBL2" ... why two regs in one row?
        dbg!(&descr);

        // TODO: Turn mode field into static' str...
        // error: implementation of `datasheet::_::_serde::Deserialize` is not general enough
        //     = note: `Record` must implement `datasheet::_::_serde::Deserialize<'0>`, for any lifetime `'0`...
        // = note: ...but `Record` actually implements `datasheet::_::_serde::Deserialize<'1>`, for some specific lifetime `'1`
        //
        // let mode = match mode {
        //     "R/W" => "read-write",
        //     "R/O" => "read-only",
        //     "W" => "write-only"
        // };

        if &mode == "R/W" {
            mode = "read-write".to_string();
        } else if &mode == "R/O" || &mode == "R" {
            mode = "read-only".to_string();
        } else if &mode == "W" {
            mode = "write-only".to_string();
        }

        dbg!(&mode);
        dbg!(&reset_value); // TODO: Disambiguate cases like "00H/01H" ... what does that mean?
    }
    Ok(())
}