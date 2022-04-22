pub mod addr_meta;
pub mod thread_meta;

// reexports
pub use addr_meta::AddrMeta;
pub use thread_meta::ThreadMeta;

use hashbrown::HashMap;
use crate::consts;

pub struct Metadata {
    next_addr_space: usize,

    // TODO make your own wrapper or find a better one instead of using Option
    //  we need option here because HashMap::new can't be called from a constant function
    addr2tid: Option<HashMap<usize, usize>>,

    tid2tmeta: Option<HashMap<usize, ThreadMeta>>,
}

impl Metadata {
    pub const fn new() -> Self {
        Metadata {
            next_addr_space: consts::FIRST_ADDR_SPACE_START,
            addr2tid: None,
            tid2tmeta: None
        }
    }

    pub fn init(&mut self) {
        self.addr2tid = Some(HashMap::new());
        self.tid2tmeta = Some(HashMap::new());
    }

    pub fn add_new_thread(&mut self, tid: usize) {
        // new thread => new associated ThreadMeta structure
        let tid2meta = self.tid2tmeta.as_mut().unwrap();
        tid2meta.insert(tid, ThreadMeta::new(self.next_addr_space));

        // new thread => new address space
        self.next_addr_space -= consts::ADDR_SPACE_SIZE;

        // new mapping: new address space -> new thread
        let addr2tid = self.addr2tid.as_mut().unwrap();
        addr2tid.insert(self.next_addr_space, tid);
    }

    pub fn get_tmeta_for_tid(&self, tid: usize) -> Option<&ThreadMeta> {
        self.tid2tmeta.as_ref().unwrap().get(&tid)
    }

    pub fn get_tmeta_for_addr(&self, addr: usize) -> Option<&ThreadMeta> {
        let tid = *self.addr2tid.as_ref().unwrap().get(&(addr & consts::ADDR_SPACE_MASK)).unwrap();
        self.get_tmeta_for_tid(tid)
    }
}
