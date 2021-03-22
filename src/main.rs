mod svd;
mod datasheet;

use datasheet::{ parse_datasheet, clean_datasheet_sections };
use svd::{ generate_svd };

fn main() {
    // Get information from datasheet
    let interrupts = parse_datasheet("datasheets/renesas/v850/pdf/nec-μPD703128.pdf", "70-72", true);
    let peripherals = parse_datasheet("datasheets/renesas/v850/pdf/nec-μPD703128.pdf", "76-82", true);
    let programmable_io = parse_datasheet("datasheets/renesas/v850/pdf/nec-μPD703128.pdf", "85-102", false);
    // let _interrupt_control = parse_datasheet("datasheets/renesas/v850/pdf/nec-μPD703128.pdf", "217-218", false);
    // let _can_registers = parse_datasheet("datasheets/renesas/v850/pdf/nec-μPD703128.pdf", "432-437", false);

    let datasheet_sections = vec!(interrupts, peripherals, programmable_io);// _interrupt_control, _can_registers);
    //dbg!(&datasheet_sections);
    let clean_datasheet = clean_datasheet_sections(datasheet_sections);

    // Serialize it into a (hopefully) well-formed SVD
    let svd_res = generate_svd(clean_datasheet);
    println!("{}", &svd_res.ok().unwrap());
}