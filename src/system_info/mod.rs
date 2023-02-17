pub mod memory;
pub mod system;
pub struct System {
    
}

#[derive(Default,Debug)]
pub struct SysMemory {
    total_mem       : u64,
    free_mem        : u64,
    avialable_mem   : u64,
    buffers         : u64,
    cached          : u64,
    swapped_cache   : u64,
}