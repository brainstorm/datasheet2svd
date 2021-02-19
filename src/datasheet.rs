//use std::fs;
use std::error::Error;
use std::process::{ Command, Output };

use csv::{ Reader, StringRecord };
use serde::{ Serialize, Deserialize };

use crate::svd::{ AddrBlock, Register, Registers, Peripheral, Peripherals };

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

/// Runs tabula PDF OCR or uses a precomputed CSV file (cached=true)
/// tabula.jar:
// wget https://github.com/tabulapdf/tabula-java/releases/download/v1.0.4/tabula-1.0.4-jar-with-dependencies.jar
pub fn run_tabula(datasheet: &str, page_range: &str, cached: bool) -> Output {
    let output;
    if cached {
        output = Command::new("java")
                    .args(&["-jar", "bin/tabula.jar", "-p", page_range, datasheet])
                    .output()
                    .expect("Fail");
    } else {
        output = Command::new("cat")
                    .args(&["build/peripherals.csv"])
                    .output()
                    .expect("Fail");
        // output = fs::read_to_string("build/peripherals.csv")
        //                 .expect("Something went wrong reading the file");
        // TODO: types... Output vs String...
    }

    // TODO: Error ctrl
    //println!("status: {}", output.status);
    //io::stderr().write_all(&output.stderr).unwrap();
        
    return output;
}

pub fn clean_peripherals(csv_data: Output) -> Result<Peripherals, Box<dyn Error>> {
    let mut rdr = Reader::from_reader(&*csv_data.stdout);
    rdr.set_headers(StringRecord::from(vec!["address", "description", "name", "mode", 
                                            "manip_1_bit", "manip_8_bit", "manip_16_bit",
                                            "reset_value"]));

    let mut peripherals_vec: Vec<Peripheral> = Vec::new();
    let mut manipsize = 8;

    for result in rdr.deserialize() {
        let record: Record = result?;

        let mut addr = record.address;
        let descr = record.description;
        let name = record.name.replace("\r", &"");
        let mut mode = record.mode;
        let _manip_1_bit = record.manip_1_bit;
        let manip_8_bit = record.manip_8_bit;
        let manip_16_bit = record.manip_16_bit;

        // Puts things to 0 if reset_value is not parsed correctly (Error kind: Empty)
        let reset_value: i64 = match record.reset_value.trim().parse::<i64>() {
            Ok(value) => value,
            Err(_err) => 0,
        };

        // Skip repeated headers over pages
        if addr == "Address" {
            continue;
        }

        // Multi-row datasheet table register definitions
        if addr == "" {
            if name == "" { continue; } // beginning of table, discard
            continue;
        }

        // Full address, i.e: FFFF EEEE to 0xFFFFEEEE
        addr.retain(|ch| !ch.is_whitespace());
        addr = String::from("0x") + &addr;

        // SVD-friendly read/write format
        if mode == "R/W" {
            mode = "read-write".to_string();
        } else if mode == "R/O" || mode == "R" {
            mode = "read-only".to_string();
        } else if mode == "W" {
            mode = "write-only".to_string();
        }

        // TODO: How can we access this register?
        //let manip_bits = ManipBits { false, false, false };
        //if manip_1_bit == "×" { manipsize = 1; }
        if manip_8_bit == "×" { manipsize = 0x8; }
        if manip_16_bit == "×" { manipsize = 0x10; }

        // TODO: PeripheralIO and Register is a 1-1 relationship right now. Explore how to generalize and improve this.
        // For instance, MCU registers have a 1-many (MCU-many regs) relationship, accomodate this function for those too?
        let register = Register {
            name: name.clone(),
            description: descr.clone(),
            addressoffset: addr.clone(),
            size: manipsize,
            access: mode.to_string(),
            resetvalue: reset_value,
            resetmask: "0xFFFFFFFF".to_string(),
            fields: vec![] // TODO: Not bothering about bitfields for now
        };

        // TODO: Again, just 1-1 peripheral to register here for MMIO, must be refactored for 1-many
        let registers = Registers { register: vec![register] };

        let addressblock = AddrBlock {
            offset: "0x0".to_string(), //addr.to_string(),
            size: manipsize.to_string(),
            usage: "mmio".to_string()
        };

        let peripheral = Peripheral {
            name: name.to_string(),
            version: "1.0".to_string(),
            description: descr.to_string(),
            groupname: "mmio".to_string(),
            baseaddress: addr.to_string(),
            addressblock: addressblock,
            // size: 16,
            // access: mode.to_string(),
            registers: registers // TODO: Still 1-1 for now, for loop for 1-many on other datasheet tables
        };

        // Accumulate peripheral entries
        peripherals_vec.push(peripheral);

        // Reset manip bits for next register
        manipsize = 0;
    }

    // Wrap on struct before shipping
    let peripherals = Peripherals {
        peripheral: peripherals_vec
    };

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
                    // Also it messes up later on with the XML:
                    //    </peripherals>
                    //    <peripherals>
                    //      SIRBL0</name>IRB0/   <---- !!!
                    //    <version>1.0</version>
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