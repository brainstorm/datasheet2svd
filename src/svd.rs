use yaserde_derive::YaSerialize;

#[derive(Default, PartialEq, Debug, YaSerialize)]
pub struct CpuDef {
    #[yaserde(child)]
    pub name: String,
    #[yaserde(child)]
    pub revision: String,
    #[yaserde(child)]
    pub endian: String, // enum {LE, BE, ME}
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
    pub resetvalue: String,
    #[yaserde(child)]
    pub resetmask: String,
    #[yaserde(child)]
    pub fields: Vec<Field>
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
pub struct Peripheral {
    #[yaserde(child)]
    pub name: String,
    #[yaserde(child)]
    pub version: String,
    #[yaserde(child)]
    pub description: String,
    #[yaserde(child)]
    pub groupname: String,
    #[yaserde(child)]
    pub baseaddress: String,
    #[yaserde(child)]
    pub size: u8,
    #[yaserde(child)]
    pub access: String,
    #[yaserde(child)]
    pub registers: Vec<Register>
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
pub struct DevAttrs {
    #[yaserde(child)]
    pub vendor: String,
    #[yaserde(child)]
    pub vendorid: String,
    #[yaserde(child)]
    pub name: String,
    #[yaserde(child)]
    pub series: String,
    #[yaserde(child)]
    pub version: String,
    #[yaserde(child)]
    pub description: String,
    #[yaserde(child)]
    pub licensetext: String,
    #[yaserde(child)]
    pub cpu: CpuDef,
    #[yaserde(child)]
    pub addressunitbits: u8,
    #[yaserde(child)]
    pub width: u8,
    #[yaserde(child)]
    pub size: u8,
    #[yaserde(child)]
    pub access: String,
    #[yaserde(child)]
    pub resetvalue: String,
    #[yaserde(child)]
    pub resetmask: String,
    #[yaserde(child)]
    pub peripherals: Vec<Peripheral>
}

#[derive(Default, PartialEq, Debug, YaSerialize)]
#[yaserde(rename = "device")]
pub struct Device {
    #[yaserde(attribute)]
    pub schemaversion: String,
    #[yaserde(attribute)]
    pub xmlns: String,
    #[yaserde(attribute)]
    pub xsnonamespaceschemalocation: String,
    #[yaserde(child)]
    pub devattributes: DevAttrs
}