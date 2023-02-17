use std::fs;

use super::{System, SysMemory};



impl System {
    pub fn get_memory() -> SysMemory{
        let mut memory = SysMemory::default();
        let contents = fs::read_to_string("/proc/meminfo");
        let contents = match contents {
            Ok(file) => file,
            Err(_e) => "".to_string()
        };

        let lines = contents.split("\n");
        for line in lines {
            let item = line.split(":");
            let key_value = item.collect::<Vec<&str>>();
            let key = key_value[0];
            let value = key_value[1];
            let value = value.trim().trim_end_matches(" KB").parse::<u64>().expect("Not a int");
            if key.eq("MemTotal") {
                memory.total_mem = value * 1000;
            } else if key.eq("MemFree") {
                memory.free_mem = value * 1000;
            } else if key.eq("MemAvialable") {
                memory.avialable_mem = value * 1000;
            } else if key.eq("Buffers") {
                memory.buffers = value * 1000;
            } else if key.eq("Cached") {
                memory.buffers = value * 1000;
            } else if key.eq("SwapCached") {
                memory.buffers = value * 1000;
            }
        }
        memory
    }

    pub fn uptime() {

    }

    pub fn shutdonw() {

    }

    


}