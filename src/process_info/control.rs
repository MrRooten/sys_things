use core::time;
use std::os::unix::prelude::FileExt;
use std::{path::PathBuf, error::Error, mem::size_of, ffi::c_void, fs::File};
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::io::SeekFrom;
use base64::{decode_config_slice, Config, decode, encode};
use libc::{ptrace, PTRACE_ATTACH, PTRACE_DETACH, PTRACE_GETREGS, user_regs_struct, PTRACE_SETREGS, PTRACE_CONT, PTRACE_POKETEXT, process_vm_readv, iovec, system, process_vm_writev, getchar};
use syscalls::SyscallArgs;

use crate::process_info::libhelper::{ptrace_write, ptrace_cont, ptrace_read, ptrace_detach};
use crate::utils::STError;
use crate::utils::func::i_to_m;

use super::{Process, Memory, Signal};


impl Process {

    pub fn send_signal(&self, sig : Signal) {
        let args = syscalls::SyscallArgs {
            arg0 : self.pid as usize,
            arg1 : sig as usize,
            arg2 : 0,
            arg3 : 0,
            arg4 : 0,
            arg5 : 0
        };
        let result;
        unsafe {
            result = syscalls::syscall(syscalls::SYS_kill,&args);
        }


    }

    fn nice(&self, level : i32) {
        
    }

    fn set_map_priv(&self) {

    }
    pub fn write_memory(&self, addr: u64,data: &Vec<u8>) -> isize {
        unsafe {
            let len = data.len();
            let arr:[u8;2] = [0x12,0x34];
            let liov = iovec {
                iov_base    : data.as_ptr() as *mut c_void,
                iov_len     : len
            };

            let riov = iovec {
                iov_base    :addr as *mut c_void,
                iov_len     : len
            };
            let mut ret = process_vm_writev(self.pid.try_into().unwrap(), &liov, 1, &riov, 1, 0);
            if ret < 0 {
                if self.is_trace == false {
                    ret = ptrace(PTRACE_ATTACH,self.pid).try_into().expect("");
                }
                std::thread::sleep(time::Duration::from_millis(100));
                if self.is_trace == false && ret < 0 {
                    return ret as isize;
                }
                if len % 8 == 0 {
                    ret = ptrace_write(self.pid, addr, data.as_ptr() as *mut c_void, len).try_into().expect("");
                } else {
                    let save = self.read_memory(addr+(len as u64), 8).unwrap();
                    ret = ptrace_write(self.pid, addr, data.as_ptr() as *mut c_void, len).try_into().expect("");
                    ret = ptrace_write(self.pid,addr+(len as u64),save.as_ptr() as *mut c_void,8).try_into().expect("");
                }
                ptrace_cont(self.pid);
                if self.is_trace == false {
                    ptrace_detach(self.pid);
                }
                return ret as isize;
            }
            ret as isize
        }
    }

    pub fn read_memory(&self, addr: u64, len: usize) -> Result<Vec<u8>,STError> {
        unsafe {
            let mut arr = vec![0 as u8; len];
            let liov = iovec {
                iov_base    : arr.as_mut_ptr() as *mut c_void,
                iov_len     : len
            };

            let riov = iovec {
                iov_base    :addr as *mut c_void,
                iov_len     : len
            };
            let ret = process_vm_readv(self.pid.try_into().unwrap(), &liov, 2, &riov, 1, 0);
            if ret < 0 {
                return Err(STError::new("Error to read Virtual Memory"));
            }
            Ok(arr)
        }
    }

    pub fn change_map_perm(&self) {

    }
    fn get_baselibc(&self) -> u64 {
        unimplemented!()
    }

    fn get_libcbase(&self) -> u64 {
        let maps = self.get_mmaps().unwrap();
        for map in maps {
            if map.pathname.contains("libc-2.33.so") {
                return map.start_addr;
            }
        }
        0
    }

    fn get_freespace(&self) -> u64 {
        let maps = self.get_mmaps().unwrap();
        for map in maps {
            if map.perm.contains("x") {
                return map.start_addr;
            }
        }

        0
    }
    fn trace_me(&mut self) {
        unsafe {
            let ret = ptrace(PTRACE_ATTACH ,self.pid);
            if ret >= 0 {
                self.is_trace = true;
            } else {
                self.is_trace = false;
            }
        }
    }

    fn detrace_me(&mut self) {
        unsafe {
            let ret = ptrace(PTRACE_DETACH ,self.pid);
            if ret >= 0 {
                self.is_trace = false;
            } else {
                self.is_trace = true;
            }
        }
    }

    pub fn inject_code(&self, code: &Vec<u8>) -> Result<u64,STError> {
        let orignal_code = [0 as u8;1024];
        unsafe {
            (i_to_m(self)).trace_me();
            
            if self.is_trace == false {
                return Err(STError::new("Error in ptrace PTRACE_ATTACH"));
            }
            std::thread::sleep(time::Duration::from_millis(1000));
            let arr = [0 as u8;size_of::<user_regs_struct>()];
            let mut user_regs = arr.as_ptr() as *mut user_regs_struct;
            let ret = ptrace(PTRACE_GETREGS, self.pid, 0, user_regs);
            if ret < 0 {
                return Err(STError::new("Error in ptrace PTRACE_GETREGS"));
            }
            let old_user_regs = *user_regs;
            let rip_addr = (*user_regs).rip;
            //Backup orignal code
            let orignal_code = self.read_memory(rip_addr, code.len()).unwrap();
            let ret = self.write_memory(rip_addr, code);
            if ret < 0 {
                return Err(STError::new("Error in write memory to rip"));
            }
            let write_size = self.write_memory(rip_addr, &orignal_code);
            ptrace_cont(self.pid);
            let ret = ptrace_cont(self.pid);
            if ret == -1 {
                return Err(STError::new("Error in ptrace_cont"));
            }
            (i_to_m(self)).detrace_me();
            Ok(rip_addr)
        }
        
    }

    fn coredump(&self, path: String) -> Result<(),STError> {
        unimplemented!()
    }
}