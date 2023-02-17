pub mod info;
pub mod control;
pub mod libhelper;
#[derive(Default)]
pub struct Process {
    pid  : u32,
    name : String,
    cmdline : String,
    memory : Memory,
    is_trace : bool
}
#[derive(Default)]
pub struct Memory {
    vm_peak              : u64,
    vm_size              : u64,
    vm_lock              : u64,
    vm_pin               : u64,
    vm_high_watermark    : u64,
    vm_resident_set_size : u64,
    rss_anonymous_memory : u64,
    rss_file_mapping     : u64,
    rss_shared_memory    : u64,
    vm_data              : u64,
    vm_stack             : u64,
    vm_execute           : u64,
    vm_library           : u64,
    vm_page_entry_table  : u64,
    vm_page_second_table : u64,
    vm_swap              : u64,
}

pub struct IoState {

}

pub struct CGroup {

}

pub struct MemoryMap {
    start_addr      : u64,
    end_addr        : u64,
    size            : u64,
    pathname        : String,
    inode           : u64,
    dev             : String,
    perm            : String
}

impl MemoryMap {
    pub fn get_start_addr(&self) -> u64 {
        self.start_addr
    }

    pub fn get_name(&self) -> &String {
        &self.pathname
    }

    pub fn get_perm(&self) -> &String {
        &self.perm
    }

    pub fn in_memory(&self, addr: u64) -> bool {
        if addr > self.start_addr && addr < self.end_addr {
            return true;
        }

        return false;
    }

    pub fn get_end_addr(&self) -> u64 {
        self.end_addr
    }
}
pub enum Signal {
        SIGHUP=1,
        SIGINT=2,
        SIGQUIT=3,
        SIGILL=4,
        SIGTRAP=5,
        SIGABRT=6,
        SIGBUS=7,
        SIGFPE=8,
        SIGKILL=9,
        SIGUSR1=10,
        SIGSEGV=11,
        SIGUSR2=12,
        SIGPIPE=13,
        SIGALRM=14,
        SIGTERM=15,
        SIGSTKFLT=16,
        SIGCHLD=17,
        SIGSTOP=19,
        SIGTSTP=20,
        SIGTTIN=21,
        SIGTTOU=22,
        SIGURG=23,
        SIGXCPU=24,
        SIGXFSZ=25,
        SIGVTALRM=26,
        SIGPROF=27,
        SIGWINCH=28,
        SIGIO=29,
        SIGPWR=30,
        SIGSYS=31
}

impl Signal {
    pub const SIGIOT: Signal = Signal::SIGABRT;
    pub const SIGPOLL: Signal = Signal::SIGIO;
    pub const SIGUNUSED: Signal = Signal::SIGSYS;
}