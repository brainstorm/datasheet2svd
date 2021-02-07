extern crate serde;
extern crate quick_xml;

use serde::Serialize;
use quick_xml::se::to_string as to_xml;

#[derive(Debug, Serialize, PartialEq)]
struct CpuDef {
    name: String,
    revision: String,
    endian: String, // enum {LE, BE, ME}
    mpupresent: bool,
    fpupresent: bool,
    //nvicpriobits: enum {8, 16, 32, 64, 128},
    vendorsystickconfig: bool
}

#[derive(Debug, Serialize, PartialEq)]
struct Field {
    name: String,
    description: String,
    bitrange: String,
    access: String,
}

#[derive(Debug, Serialize, PartialEq)]
struct Register {
    name: String,
    description: String,
    addressoffset: String,
    size: u8,
    access: String,
    resetvalue: String,
    resetmask: String,
    fields: Vec<Field>
}

#[derive(Debug, Serialize, PartialEq)]
struct Peripheral {
    name: String,
    version: String,
    description: String,
    groupname: String,
    baseaddress: String,
    size: u8,
    access: String,
    registers: Vec<Register>
}

#[derive(Debug, Serialize, PartialEq)]
struct DevAttrs {
    vendor: String,
    vendorid: String,
    name: String,
    series: String,
    version: String,
    description: String,
    licensetext: String,
    cpu: CpuDef,
    addressunitbits: u8,
    width: u8,
    size: u8,
    access: String,
    resetvalue: String,
    resetmask: String,
    peripherals: Vec<Peripheral>
}

#[derive(Debug, Serialize, PartialEq)]
struct Device {
    schemaversion: String,
    xmlns: String,
    xsnonamespaceschemalocation: String,
    devattributes: DevAttrs
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename = "xml", default)]
struct XmlRoot {
    version: String,
    encoding: String,
    device: Device
}

fn main() {

    let mut vec_peripherals: Vec<Peripheral> = Vec::new();
    let mut vec_registers: Vec<Register> =  Vec::new();
    let vec_fields: Vec<Field> = Vec::new();

    let register = Register {
        name: "PRCMD".to_string(),
        description: "This command register (PRCMD) is to protect the registers that may have a significant influence on the application system (PSC, PSM) from an inadvertent write access, so that the system does not stop in case of a program hang-up.".to_string(),
        addressoffset: "0xFFFFF1FC".to_string(),
        size: 8,
        access: "read-write".to_string(),
        resetvalue: "0x0000".to_string(),
        resetmask: "0xFFFF".to_string(),
        fields: vec_fields
    };
    vec_registers.push(register);

    let peripheral = Peripheral {
        name: "Specific Registers".to_string(),
        version: "1.0".to_string(),
        description: "Specific Registers".to_string(),
        groupname: "MCU".to_string(),
        baseaddress: "0xFFFFF1FC".to_string(),
        size: 16,
        access: "read-write".to_string(),
        registers: vec_registers
    };
    vec_peripherals.push(peripheral);

    let cpu_def = CpuDef {
        name: "V850".to_string(),
        revision: "r1".to_string(),
        endian: "LE".to_string(), // enum {LE, BE, ME}
        mpupresent: false,
        fpupresent: false,
        //nvicpriobits: enum {8, 16, 32, 64, 128},
        vendorsystickconfig: false    
    };

    let dev_attrs = DevAttrs {
        vendor: "Renesas".to_string(),
        vendorid: "Renesas".to_string(),
        name: "V850".to_string(),
        series: "E1/E2/CA2".to_string(),
        version: "1.2".to_string(),
        description: "NEC/Renesas V850 automotive grade ICs".to_string(),
        licensetext: "GPLv3".to_string(),
        cpu: cpu_def,
        addressunitbits: 8,
        width: 32,
        size: 32,
        access: "read-write".to_string(),
        resetvalue: "0x00000000".to_string(),
        resetmask: "0xFFFFFFFF".to_string(),
        peripherals: vec_peripherals
    };

    let dev = Device {  schemaversion: "foo".to_string(),
                        xmlns: "http://www.w3.org/2001/XMLSchema-instance".to_string(),
                        xsnonamespaceschemalocation: "CMSIS-SVD.xsd".to_string(),
                        devattributes: dev_attrs
                    };

    let xml_str = XmlRoot { version: "1.0".to_string(),
                            encoding: "utf-8".to_string(),
                            device: dev
                        };
    
    println!("{:?}", to_xml(&xml_str).ok().unwrap());
}