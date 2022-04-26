pub struct AddrMeta {
    pub size: usize,
}

impl AddrMeta {
    pub fn new(size: usize) -> Self {
        AddrMeta { size }
    }
}
