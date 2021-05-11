use yaserde_derive::YaSerialize;

pub fn generate_svd(peripherals: Vec<Peripherals>) -> Result<String, String> {
    let cpu_def = CpuDef {
        name: "V850".to_string(),
        revision: "r1".to_string(),
        endian: "LE".to_string(), // TODO: enum {LE, BE, ME}?
        mpupresent: false,
        fpupresent: false,
        nvicpriobits: 32,
        vendorsystickconfig: false    
    };

    let dev = Device {  
        vendor: "Renesas".to_string(),
        vendor_id: "Renesas".to_string(),
        name: "V850".to_string(),
        series: "E1/E2/CA2".to_string(),
        version: "1.2".to_string(),
        description: "NEC/Renesas V850 automotive grade ICs".to_string(),
        //licensetext: "GPLv3".to_string(),
        cpu: cpu_def,
        addressunitbits: 8,
        width: 32,
        size: 0x6, // TODO: Determine what this size parm does vs BlockAddress at reg level.
        access: "read-write".to_string(),
        resetvalue: 0x0,
        resetmask: "0xFFFFFFFF".to_string(),
        peripherals: peripherals
    };

    // Return pretty printed XML
    let yaserde_cfg = yaserde::ser::Config{
        perform_indent: true,
        .. Default::default()
    };

    return yaserde::ser::to_string_with_config(&dev, &yaserde_cfg);
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
pub struct CpuDef {
    #[yaserde(child)]
    pub name: String,
    #[yaserde(child)]
    pub revision: String,
    #[yaserde(child)]
    pub endian: String, // TODO: enum {LE, BE, ME}
    #[yaserde(child)]
    pub mpupresent: bool,
    #[yaserde(child)]
    pub fpupresent: bool,
    #[yaserde(child)]
    pub nvicpriobits: u8,
    #[yaserde(child)]
    pub vendorsystickconfig: bool
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
pub struct Field {
    pub name: String,
    #[yaserde(child)]
    pub description: String,
    #[yaserde(child)]
    pub bitrange: String,
    #[yaserde(child)]
    pub access: String,
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "register")]
pub struct Register {
    #[yaserde(child)]
    pub name: String,
    #[yaserde(child)]
    pub description: String,
    #[yaserde(child, rename = "addressOffset")]
    pub addressoffset: String,
    #[yaserde(child)]
    pub size: u8,
    #[yaserde(child)]
    pub access: String,
    #[yaserde(child, rename = "resetValue") ]
    pub resetvalue: i64,
    #[yaserde(child, rename = "resetMask")]
    pub resetmask: String,
    #[yaserde(child)]
    pub fields: Vec<Field>
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "addressBlock")]
pub struct AddrBlock {
    #[yaserde(child)]
    pub offset: String,
    #[yaserde(child)]
    pub size: String,
    #[yaserde(child)]
    pub usage: String
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "peripheral")]
pub struct Peripheral {
    #[yaserde(child)]
    pub name: String,
    #[yaserde(child)]
    pub version: String,
    #[yaserde(child)]
    pub description: String,
    #[yaserde(child)]
    pub groupname: String,
    #[yaserde(child, rename="baseAddress")]
    pub baseaddress: String,
    #[yaserde(child, rename="addressBlock")]
    pub addressblock: AddrBlock,
    #[yaserde(child)]
    pub registers: Registers
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "peripheral")]
pub struct Peripherals {
    pub peripheral: Vec<Peripheral>,
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "register")]
pub struct Registers {
    #[yaserde(rename = "register")]
    pub registers: Vec<Register>,
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
#[yaserde(
    rename = "device",
    namespace = "xs: http://www.w3.org/2001/XMLSchema-instance",
    // TODO: 
    //  1) How to add the following namespace without overriding the first?
    //  2) How do I get rid of the xmls: prefix for the second namespace?
    //namespace = "xs: noNamespaceSchemaLocation=CMSIS-SVD.xsd"
)]
pub struct Device {
    // #[yaserde(attribute)]
    // pub schemaversion: String,
    // #[yaserde(attribute)]
    // pub xmlns: String,
    // #[yaserde(attribute)]
    // pub xsnonamespaceschemalocation: String,
    // #[yaserde(child)]
    #[yaserde(child)]
    pub vendor: String,
    #[yaserde(child)]
    pub vendor_id: String,
    #[yaserde(child)]
    pub name: String,
    #[yaserde(child)]
    pub series: String,
    #[yaserde(child)]
    pub version: String,
    #[yaserde(child)]
    pub description: String,
    // #[yaserde(child)]
    // pub licensetext: String,
    #[yaserde(child)]
    pub cpu: CpuDef,
    #[yaserde(child, rename = "addressUnitBits")]
    pub addressunitbits: u8,
    #[yaserde(child)]
    pub width: u8,
    #[yaserde(child)]
    pub size: u8,
    #[yaserde(child)]
    pub access: String,
    #[yaserde(child)]
    pub resetvalue: i64,
    #[yaserde(child)]
    pub resetmask: String,
    #[yaserde(child)]
    pub peripherals: Vec<Peripherals>
}