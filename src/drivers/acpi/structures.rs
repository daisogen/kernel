pub const SIGNATURE: &str = "RSD PTR ";

#[repr(C, packed)]
pub struct RSDPDescriptor {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oemid: [u8; 6],
    pub revision: u8,
    pub rsdt: u32,
}

#[repr(C, packed)]
pub struct RSDPDescriptor20 {
    v1: RSDPDescriptor,
    pub length: u32,
    pub xsdt: u64,
    pub checksum2: u8,
    pub reserved: [u8; 3],
}

#[repr(C, packed)]
pub struct SDTHeader {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oemid: [u8; 6],
    pub oemtableid: [u8; 8],
    pub oemrevision: u32,
    pub creatorid: u32,
    pub creatorrevision: u32,
}
