/// This class contains highly specific parsing for datasheet tables (in this case, for Renesas V850 IC)
use std::error::Error;
use std::process::{ Command, Output };

use csv::{ Reader, StringRecord };
use serde::{ Serialize, Deserialize };

use crate::svd::{ AddrBlock, Register, Registers, Peripheral, Peripherals };

#[derive(Debug, Serialize, Deserialize)]
struct PeripheralDatasheetColumn {
    address: String,
    description: String,
    name: String,
    mode: String,
    manip_1_bit: String,
    manip_8_bit: String,
    manip_16_bit: String,
    reset_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct InterruptDatasheetColumn {
    address: String,
    interrupt_exception_source: String,
}

/// Runs tabula PDF OCR or uses a precomputed CSV file (cached=true)
/// tabula.jar:
// wget https://github.com/tabulapdf/tabula-java/releases/download/v1.0.4/tabula-1.0.4-jar-with-dependencies.jar
pub fn parse_datasheet(datasheet: &str, page_range: &str, cached: bool) -> Output {
    let output;
    if !cached {
        output = Command::new("java")
                    .args(&["-jar", "bin/tabula.jar", "-p", page_range, datasheet])
                    .output()
                    .expect("Fail");
    } else {
        //dbg!("Getting CSV instead of PDF");
        output = Command::new("cat")
                    .args(&[format!("datasheets/renesas/v850/csv/{}.csv", page_range)])
                    .output()
                    .expect("Fail");
    }

    return output;
}

/// Dispatch heterogeneous CSV data cleaning functions
pub fn clean_datasheet_sections(sections: Vec<Output>) -> Result<Peripherals, Box<dyn Error>> {
    let interrupts = clean_interrupts(sections[0].clone())?;
    let mmio = clean_mmio(sections[1].clone(), "mmio".to_string())?;
    let prog_io = clean_mmio(sections[2].clone(), "pmmio".to_string())?;

    let peripherals = Peripherals::new(interrupts);
    Ok(peripherals.add(mmio)
                  .add(prog_io))
}

pub fn clean_interrupts(section: Output) -> Result<Vec<Peripheral>, Box<dyn Error>> {
    let mut rdr = Reader::from_reader(&*section.stdout);
    rdr.set_headers(StringRecord::from(vec!["address", "interrupt_exception_source"]));

    let mut peripherals_vec: Vec<Peripheral> = Vec::new();
    let mut registers_vec: Vec<Register> = Vec::new();

    for result in rdr.deserialize() {
        // TODO: Skip invalid text prelude before the table
        // TODO: Better types and error control here
        // let record: InterruptDatasheetColumn = match result {
        //     Ok(res) => res,
        //     Err(_) => break
        // };
        let record: InterruptDatasheetColumn = result?;

        let mut addr = record.address;
        let interrupt = record.interrupt_exception_source;

        // Full address, i.e: FFFF EEEE to 0xFFFFEEEE
        addr.retain(|ch| !ch.is_whitespace());
        addr = String::from("0x") + &addr;

        let register = Register {
            name: interrupt.clone(),
            description: interrupt.clone(),
            addressoffset: addr.clone(),
            size: 0x10,
            access: "read-write".to_string(),
            resetvalue: 0x0,
            resetmask: "0xFFFFFFFF".to_string(),
            fields: vec![] // TODO: Not bothering about bitfields for now
        };

        registers_vec.push(register);
    }

    let addressblock = AddrBlock {
        offset: "0x0".to_string(), //addr.to_string(),
        size: "0x00000470".to_string(),
        usage: "irq".to_string()
    };

    let peripheral = Peripheral {
        name: "irq".to_string(),
        version: "1.0".to_string(),
        description: "Interrupts and exception table".to_string(),
        groupname: "irq".to_string(),
        baseaddress: "0x00000000".to_string(),
        addressblock: addressblock,
        // size: 16,
        // access: mode.to_string(),
        registers: Registers { registers: registers_vec }
    };

    // Accumulate peripheral entries
    peripherals_vec.push(peripheral);
    Ok(peripherals_vec)

}

pub fn clean_mmio(section: Output, groupname: String) -> Result<Vec<Peripheral>, Box<dyn Error>> {
    let mut rdr = Reader::from_reader(&*section.stdout);
    rdr.set_headers(StringRecord::from(vec!["address", "description", "name", "mode", 
                                            "manip_1_bit", "manip_8_bit", "manip_16_bit",
                                            "reset_value"]));

    let mut peripherals_vec: Vec<Peripheral> = Vec::new();
    let mut registers_vec: Vec<Register> = Vec::new();
    let mut manipsize = 8;

    for result in rdr.deserialize() {
        let record: PeripheralDatasheetColumn = result?;

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

        if manip_8_bit == "×" { manipsize = 0x8; }
        if manip_16_bit == "×" { manipsize = 0x10; }

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

        registers_vec.push(register);

        // Reset manip bits for next register
        manipsize = 0;
    }

    // TODO: This needs to be updated for each register but cannot be accumulated like registers_vec... how to fix then?
    let addressblock = AddrBlock {
        offset: "0x0".to_string(), //addr.to_string(),
        size: manipsize.to_string(),
        usage: "mmio".to_string()
    };

    let peripheral = Peripheral {
        name: groupname.clone(),
        version: "1.0".to_string(),
        description: "Memory Mapped IO, peripherals".to_string(),
        groupname: groupname.clone(),
        baseaddress: "0xFFFFF000".to_string(),
        addressblock: addressblock,
        // size: 16,
        // access: mode.to_string(),
        registers: Registers { registers: registers_vec }
    };

    // Accumulate peripheral entries
    peripherals_vec.push(peripheral);

    Ok(peripherals_vec)
}
