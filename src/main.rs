mod svd;
mod datasheet;

use svd::{Peripheral,CpuDef, DevAttrs, Device};
use datasheet::{run_tabula, clean_peripherals};

fn populate_svd_struct(peripherals: Vec<Peripheral>) -> Result<String, String> {
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
        peripherals: peripherals
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
    let _interrupts = run_tabula("datasheets/nec-μPD703128.pdf", "70-72");
    let peripherals = run_tabula("datasheets/nec-μPD703128.pdf", "76-82");
    let _programmable_io = run_tabula("datasheets/nec-μPD703128.pdf", "85-102");
    let _interrupt_control = run_tabula("datasheets/nec-μPD703128.pdf", "217-218");
    let _can_registers = run_tabula("datasheets/nec-μPD703128.pdf", "432-437");

    let clean = clean_peripherals(peripherals);

    // Serialize it into a hopefully well-formed SVD
    let svd_res = populate_svd_struct(clean.unwrap());
    println!("{}", &svd_res.ok().unwrap());
}