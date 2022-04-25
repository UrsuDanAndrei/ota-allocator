pub struct AddrMeta {
    pub(crate) size: usize
}

impl AddrMeta {
    pub fn new(size: usize) -> Self {
        AddrMeta {
            size
        }
    }
}
