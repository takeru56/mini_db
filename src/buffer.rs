use std::cell::{Cell, RefCell};
use std::rc::Rc;

use crate::disk::{DiskManagere, PageId, PAGE_SIZE};

pub type Page = [u8; PAGE_SIZE];

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub struct BufferId(usize);

#[derive(Debug)]
pub struct Buffer {
    pub page_id: PageId,
    pub page: RefCell<Page>,
    pub is_dirty: Cell<bool>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            page_id: Default::default(),
            page: RefCell::new([0u8; PAGE_SIZE]),
            is_dirty: Cell::new(false),
        }
    }
}

#[defive(Debug, Default)]
pub struct Frame {
    usage_count: u64,
    buffer: Rc<Buffer>, // Bufferへの参照が破棄されたタイミングでBufferを解放する
}

pub struct BufferPool {
    buffers: Vec<Frame>,
    next_victim_id: BufferId,
}

impl BufferPool {
    pub fn new(pool_size: usize) -> Self {
        let mut buffers = vec![];
        buffers.resize_with(pool_size, Default::default); // Vec<Frame>をdefaultでpool_size埋める
        let next_victim_id = BufferId::default();
        Self {
            buffers,
            next_victim_id,
        }
    }

    fn size(&mut self) -> usize {
        self.buffers.len();
    }

    // TODO:実装
    // fn evict(&mut self) -> Option<BufferId> {
    // }
}

pub struct BufferPoolManager {
    disk: DiskManager,
    pool: BufferPool,
    page_table: HashMap<PageId, BufferId>,
}

impl BufferPoolManager {
    pub fn new(disk: DiskManagere, pool: BufferPool) -> Self {
        let page_table = HashMap::new();
        Self {
            disk,
            pool,
            page_table,
        }
    }
}
