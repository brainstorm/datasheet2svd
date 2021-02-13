use std::error::Error;
use std::process::{ Command, Output };

use csv::{ Reader, StringRecord };
use serde::{ Serialize, Deserialize };

use crate::svd::{ Register, Peripheral };

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

/// Read datasheet information needed to build the SVD from tabula's output or
/// provided manually.
// fn read_device_attrs_from_csv() {
//     unimplemented!();
// }

pub fn clean_peripherals(csv_data: Output) -> Result<Vec<Peripheral>, Box<dyn Error>> {
    let mut rdr = Reader::from_reader(&*csv_data.stdout);
    rdr.set_headers(StringRecord::from(vec!["address", "description", "name", "mode", 
                                            "manip_1_bit", "manip_8_bit", "manip_16_bit",
                                            "reset_value"]));

    let mut peripherals: Vec<Peripheral> = Vec::new();

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

        // Multi-row datasheet table register definitions
        if &addr == "" {
            if &name == "" { continue; } // beginning of table, discard
            continue;
        }

        // Full address, i.e: FFFF EEEE to FFFFEEEE
        addr.retain(|ch| !ch.is_whitespace());

        if &mode == "R/W" {
            mode = "read-write".to_string();
        } else if &mode == "R/O" || &mode == "R" {
            mode = "read-only".to_string();
        } else if &mode == "W" {
            mode = "write-only".to_string();
        }

        // Populate register and peripheral structs w/ attributes from datasheet
        // TODO: PeripheralIO and Register is a 1-1 relationship right now. Explore how to generalize and improve this.
        // For instance, MCU registers have a 1-many (MCU-many regs) relationship, accomodate this function for those too?
        let register = Register {
            name: name.clone(),
            description: descr.clone(),
            addressoffset: addr.clone(),
            size: 8,
            access: mode.clone(),
            resetvalue: reset_value.clone(),
            resetmask: "0xFFFF".to_string(),
            fields: vec![] // TODO: Not bothering about bitfields for now
        };

        let peripheral = Peripheral {
            name: name,
            version: "1.0".to_string(),
            description: descr,
            groupname: "io".to_string(),
            baseaddress: addr,
            size: 16,
            access: mode,
            registers: vec![register]
        };

        peripherals.push(peripheral);
    }

    return Result::Ok(peripherals);
}

// fn debug_parse_peripherals() {
//     // Handle multi-row descriptions for registers
//     // TODO: Handle shifting of values even more gracefully, i.e:
//     //
//     // FFFF F640,Timer mode register,TMGM0,R/W,×,×,×,0000H
//     // "",,TMGM0L,R/W,×,×,,00H
//     // FFFF F641,TMGM0H,R/W,×,×,,00H,   <--- Detect this and remember previous record
//     // FFFF F642,Channel mode register,TMGCM0,R/W,×,×,×,0000H
//     // "",,TMGCM0L,R/W,×,×,,00H
//     // FFFF F643,TMGCM0H,R/W,×,×,,00H,
//     //

//     if &addr == "" {
//         if &name == "" { continue; } // beginning of table, discard
//         dbg!("WARNING: Multi-row register definition");
//         dbg!(&name);
//         dbg!(&descr);
//         dbg!(&mode);
//         dbg!(&reset_value);
//         continue;
//     }

//     dbg!(&addr);
//     dbg!(&name); // TODO: Disambiguate cases like "SIRB2/ SIRBL2" ... why two regs in one row?
//     dbg!(&descr);

//     // TODO: Turn mode field into static' str...
//     // error: implementation of `datasheet::_::_serde::Deserialize` is not general enough
//     //     = note: `Record` must implement `datasheet::_::_serde::Deserialize<'0>`, for any lifetime `'0`...
//     // = note: ...but `Record` actually implements `datasheet::_::_serde::Deserialize<'1>`, for some specific lifetime `'1`
//     //
//     // let mode = match mode {
//     //     "R/W" => "read-write",
//     //     "R/O" => "read-only",
//     //     "W" => "write-only"
//     // };

//     dbg!(&mode);
//     dbg!(&reset_value); // TODO: Disambiguate cases like "00H/01H" ... what does that mean?
// }