use std::{ffi::CString, mem::size_of, error::Error};

use libc::{c_void, PTRACE_POKETEXT, ptrace, PTRACE_CONT, PTRACE_PEEKTEXT, user_regs_struct, PTRACE_GETREGS, PTRACE_SETREGS, PTRACE_DETACH, PTRACE_POKEDATA, c_uint};

use crate::{system_info::memory, utils::func::{m_to_i, i_to_m}};



pub fn get_u64(arr: &[u8],i: usize,j: usize) -> u64 {
    let mut ret: u64 = 0;
    let mut k = 0;
    let mut i2 = i;
    while i2 < j && k < 8 && i2 < arr.len() {
        ret += (arr[i2] as u64) << (56-k*8);
        k += 1;
        i2 += 1;
    }
    ret
}


pub fn transfer(arr: &[u8]) -> &[u32] {
    unsafe {
        std::mem::transmute(arr)
    }
}

pub unsafe fn ptrace_write(pid: u32, addr: u64, vptr: *const c_void, len: usize) -> i64 {
    let mut byte_count = 0;
    let mut word: u32 = 0;
    let mut data = vptr as u64;
    let mut arr = vptr as *const u8;
    let data = std::slice::from_raw_parts(arr, len);
    let save = [0 as u8;8];
    ptrace_read(pid,addr+(len as u64),save.as_ptr() as *const c_void,8);
    while byte_count < len  {
        let word = get_u64(data, byte_count, byte_count+8);
        let ret = ptrace(PTRACE_POKEDATA, pid, (addr+byte_count as u64) as *const c_void, u64::to_be(word) as *const c_void);
        if ret < 0 {
            return ret;
        }
        byte_count += 8;
    }
    return byte_count.try_into().unwrap();
}

pub unsafe fn ptrace_cont(pid: u32) -> i64 {
    let ret = ptrace(PTRACE_CONT, pid, 0, 0);
    return ret;
}

pub unsafe fn ptrace_read(pid: u32, addr: u64,vptr: *const c_void, len: usize) -> i64 {
    let mut byte_count = 0;
    let mut word: u32 = 0;
    let mut ptr = vptr as *const u8;
    let mut i = 0;
    while byte_count < len {
        let p_word = (&word as *const u32) as *mut c_void;
        let ret = ptrace(PTRACE_PEEKTEXT, pid, (addr + byte_count as u64) as *const c_void,vptr);
        if ret == -1 {
            return ret;
        }
        byte_count += std::mem::size_of_val(&word);
    }
    return 0;
}

pub unsafe fn ptrace_getregs(pid: u32) -> user_regs_struct {
    let arr = [0 as u8;size_of::<user_regs_struct>()];
    let mut user_regs = arr.as_ptr() as *mut user_regs_struct;
    let ret = ptrace(PTRACE_GETREGS, pid, 0, user_regs);
    *user_regs
}

pub unsafe fn ptrace_setregs(pid: u32, regs: &user_regs_struct) {
    ptrace(PTRACE_SETREGS, pid, 0, (regs) as *const user_regs_struct);
}

pub unsafe fn ptrace_detach(pid: u32) {
    ptrace(PTRACE_DETACH, pid);
}