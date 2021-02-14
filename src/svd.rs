use yaserde_derive::YaSerialize;

// #[derive(Default, PartialEq, Debug, YaSerialize)]
// pub struct CpuDef {
//     #[yaserde(child)]
//     pub name: String,
//     #[yaserde(child)]
//     pub revision: String,
//     #[yaserde(child)]
//     pub endian: String, // enum {LE, BE, ME}
//     #[yaserde(child)]
//     pub mpupresent: bool,
//     #[yaserde(child)]
//     pub fpupresent: bool,
//     #[yaserde(child)]
//     pub nvicpriobits: u8,
//     #[yaserde(child)]
//     pub vendorsystickconfig: bool
// }

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
pub struct Register {
    #[yaserde(child)]
    pub name: String,
    #[yaserde(child)]
    pub description: String,
    #[yaserde(child)]
    pub addressoffset: String,
    #[yaserde(child)]
    pub size: u8,
    #[yaserde(child)]
    pub access: String,
    #[yaserde(child)]
    pub resetvalue: i64,
    #[yaserde(child)]
    pub resetmask: i64,
    #[yaserde(child)]
    pub fields: Vec<Field>
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
    #[yaserde(child)]
    pub size: u8,
    #[yaserde(child)]
    pub access: String,
    #[yaserde(child)]
    pub registers: Vec<Register>
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "peripheral")]
pub struct Peripherals {
    pub peripheral: Vec<Peripheral>,
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
    // #[yaserde(child)]
    // pub cpu: CpuDef,
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
    pub resetmask: i64,
    #[yaserde(child)]
    pub peripherals: Peripherals
}