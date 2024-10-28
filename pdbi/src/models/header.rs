pub struct PDB_SIGNATURE {
    pub m_str: [u8; 0x20]
}

pub struct PDB_HEADER {
    pub m_signature: PDB_SIGNATURE,
    pub m_pageSize: u32,
    pub m_unknown: u32,
    pub m_filePages: u32,
    pub m_rootStreamSize: u32,
    pub m_reserved: u32,
    pub m_rootPageIndex: u32
}
