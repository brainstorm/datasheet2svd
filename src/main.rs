mod svd;
mod datasheet;

use svd::{Register, Peripheral, Field, CpuDef, DevAttrs, Device};
use datasheet::{run_tabula, clean_peripherals};

fn populate_svd_struct() -> Result<String, String> {
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
        nvicpriobits: 32,
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
    
    // Return pretty printed XML
    let yaserde_cfg = yaserde::ser::Config{
        perform_indent: true,
        .. Default::default()
    };

    return yaserde::ser::to_string_with_config(&dev, &yaserde_cfg);
}

fn main() {
    // Get information from datasheet
    let peripherals = run_tabula("datasheets/nec-μPD703128.pdf", "76-82");
    let clean = clean_peripherals(peripherals);
    //println!("{:#?}", &clean);

    // Serialize it into a well-formed SVD
    // let svd_res = populate_svd_struct();
    println!("{}", &clean.ok().unwrap());
}