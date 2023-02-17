use std::{fs, error::Error};
use std::path::{Path, PathBuf};

use crate::utils::STError;
use crate::utils::func::i_to_m;
use crate::{account_info::User, file_info::File};

use super::{Memory, Process, MemoryMap};

impl Process {
    pub fn list_process_in_proc() -> Vec<Process>{
        let path = Path::new("/proc");
        let mut res = vec![];
        for entry in fs::read_dir(path).expect("Unable to list") {
            let entry = entry.expect("unable to get entry");
            //println!("{:?}", entry.file_name());
            let s = entry.file_name().into_string();
            let s = match s {
                Ok(file) => file,
                Err(error) => {
                    continue;
                }
            };

            let s1 = String::from(s);
            let test = s1.parse::<u32>();
            let pid = match test {
                Ok(ok) => ok,
                Err(e) => {
                    continue;
                }
            };

            res.push(Process::new(pid));
        }
        res
    }

    pub fn new(pid: u32) -> Process {
        Process {
            pid: pid,
            is_trace : false,
            ..Default::default()
        }
    }

    pub fn get_pid(&self) -> u32 {
        self.pid
    }

    pub fn find_processes(name: String) -> Vec<Process> {
        let path = Path::new("/proc");
        let mut res = vec![];
        for entry in fs::read_dir(path).expect("Unable to list") {
            let entry = entry.expect("unable to get entry");
            //println!("{:?}", entry.file_name());
            let s = entry.file_name().into_string();
            let s = match s {
                Ok(file) => file,
                Err(error) => {
                    continue;
                }
            };

            let s1 = String::from(s);
            let test = s1.parse::<u32>();
            let pid = match test {
                Ok(ok) => ok,
                Err(e) => {
                    continue;
                }
            };

            let p = Process::new(pid);
            let p_name = (p.get_name()).to_string().trim().to_string();
            if name.eq(&p_name) {
                res.push(p);
            }
        }
        res
    }

    pub fn get_name(&self) -> &String {
        let comm_file = format!("/proc/{}/comm", self.pid);
        let contents = fs::read_to_string(comm_file);
        let contents = match contents {
            Ok(file) => file,
            Err(_e) => "".to_string(),
        };
        i_to_m(self).name = contents;
        &self.name
    }

    pub fn get_cmdline(&mut self) -> &String {
        let comm_file = format!("/proc/{}/cmdline", self.pid);
        let contents = fs::read_to_string(comm_file);
        let contents = match contents {
            Ok(file) => file,
            Err(_e) => "".to_string(),
        };
        self.cmdline = contents;
        &self.cmdline
    }

    pub fn get_user(&self) -> User {
        let uid = self.get_status(&"Uid".to_string());

        let uid = uid.split("\t").collect::<Vec<&str>>();
        println!("uid:{:?}", uid);
        let uid = uid[0].parse::<u32>().expect("Not a valid number");
        User::new(uid)
    }

    fn get_stat(&self, key: &String) -> String {
        let stat_file = format!("/proc/{}/stat", self.pid);
        let contents = fs::read_to_string(stat_file);
        let contents = match contents {
            Ok(file) => file,
            Err(_e) => _e.to_string(),
        };
        if contents.len() == 0 {
            return String::from("");
        }

        let values = contents.split(" ");
        let values = values.collect::<Vec<&str>>();

        if key.eq("pid") {
            return values[0].to_string();
        } else if key.eq("comm") {
            return values[1].to_string();
        } else if key.eq("state") {
            return values[2].to_string();
        } else if key.eq("ppid") {
            return values[3].to_string();
        } else if key.eq("pgrp") {
            return values[4].to_string();
        } else if key.eq("session") {
            return values[5].to_string();
        } else if key.eq("tty_nr") {
            return values[6].to_string();
        } else if key.eq("tpgid") {
            return values[7].to_string();
        } else if key.eq("flags") {
            return values[8].to_string();
        } else if key.eq("minflt") {
            return values[9].to_string();
        } else if key.eq("cminflt") {
            return values[10].to_string();
        } else if key.eq("majflt") {
            return values[11].to_string();
        } else if key.eq("cmaflt") {
            return values[12].to_string();
        } else if key.eq("utime") {
            return values[13].to_string();
        } else if key.eq("stime") {
            return values[14].to_string();
        } else if key.eq("cutime") {
            return values[15].to_string();
        } else if key.eq("cstime") {
            return values[16].to_string();
        } else if key.eq("priority") {
            return values[17].to_string();
        } else if key.eq("nice") {
            return values[18].to_string();
        } else if key.eq("num_threads") {
            return values[19].to_string();
        } else if key.eq("itrealvalue") {
            return values[20].to_string();
        } else if key.eq("starttime") {
            return values[21].to_string();
        } else if key.eq("vsize") {
            return values[22].to_string();
        } else if key.eq("rss") {
            return values[23].to_string();
        } else if key.eq("rsslim") {
            return values[24].to_string();
        } else if key.eq("startcode") {
            return values[25].to_string();
        } else if key.eq("endcode") {
            return values[26].to_string();
        } else if key.eq("startstack") {
            return values[27].to_string();
        } else if key.eq("kstkesp") {
            return values[28].to_string();
        } else if key.eq("kstkeip") {
            return values[29].to_string();
        } else if key.eq("signal") {
            return values[30].to_string();
        } else if key.eq("blocked") {
            return values[31].to_string();
        } else if key.eq("sigignore") {
            return values[32].to_string();
        } else if key.eq("sigcatch") {
            return values[33].to_string();
        } else if key.eq("wchan") {
            return values[34].to_string();
        } else if key.eq("nswap") {
            return values[35].to_string();
        } else if key.eq("cnswap") {
            return values[36].to_string();
        } else if key.eq("exit_signal") {
            return values[37].to_string();
        } else if key.eq("processor") {
            return values[38].to_string();
        } else if key.eq("rt_prority") {
            return values[39].to_string();
        } else if key.eq("policy") {
            return values[40].to_string();
        } else if key.eq("delayacct_blkio_ticks") {
            return values[41].to_string();
        } else if key.eq("guest_time") {
            return values[42].to_string();
        } else if key.eq("cguest_time") {
            return values[43].to_string();
        } else if key.eq("start_data") {
            return values[44].to_string();
        } else if key.eq("end_data") {
            return values[45].to_string();
        } else if key.eq("start_brk") {
            return values[46].to_string();
        } else if key.eq("arg_start") {
            return values[47].to_string();
        } else if key.eq("arg_end") {
            return values[48].to_string();
        } else if key.eq("env_start") {
            return values[49].to_string();
        } else if key.eq("env_end") {
            return values[50].to_string();
        } else if key.eq("exit_code") {
            return values[51].to_string();
        }
        "".to_string()
    }

    fn get_status(&self, key: &String) -> String {
        let status_file = format!("/proc/{}/status", self.pid);
        let contents = fs::read_to_string(status_file).expect("Error");

        let lines = contents.split("\n");
        for line in lines {
            let item = line.split(":");
            let key_value = item.collect::<Vec<&str>>();
            let key_s = key_value[0];
            let value = key_value[1];
            if key_s.to_string().trim().eq(key) {
                return value.trim().to_string();
            }
        }
        "".to_string()
    }

    pub fn get_state(&mut self) -> String {
        self.get_stat(&"state".to_string())
    }

    pub fn get_memory(&mut self) -> &Memory {
        let status_file = format!("/proc/{}/comm", self.pid);
        let contents = fs::read_to_string(status_file);
        let contents = match contents {
            Ok(file) => file,
            Err(_e) => "".to_string(),
        };

        let lines = contents.split("\n");
        for line in lines {
            let item = line.split(":");
            let key_value = item.collect::<Vec<&str>>();
            let key = key_value[0].trim().to_string();
            let value = key_value[1].trim().to_string();
            if (key.eq("VmPeak")) {}
        }
        &self.memory
    }

    pub fn get_load_libs(&self) {}

    pub fn get_netinfo(&self) {}

    pub fn get_filepath(&self) -> Result<PathBuf,STError> {
        let path = format!("/proc/{}/exe",self.pid);
        let f = fs::read_link(path);
        let f = match f {
            Ok(_f) => _f,
            Err(_e) => {
                return Err(STError::from(Box::new(_e)));
            }
        };

        Ok(f)
    }

    pub fn get_subtasks(&self) {}

    pub fn get_open_files(&self) -> Result<Vec<File>,STError>{
        let mut result: Vec<File> = Vec::default();
        let fd_dir = format!("/proc/{}/fd/", self.pid);
        let fds = fs::read_dir(fd_dir);
        let fds = match fds {
            Ok(_fds) => _fds,
            Err(error) => {
                return Err(STError::from(Box::new(error)));
            }
        };

        for fd in fds {
            let fd = match fd {
                Ok(_fds) => _fds,
                Err(error) => {
                    continue;
                }
            };
            let file_path = fd.path();
            let i_fd = fd
                .file_name()
                .to_str()
                .expect("Not a valid fd")
                .parse::<i32>()
                .expect("Not a valid number");
            if i_fd == 0 || i_fd == 1 || i_fd == 2 {
                continue;
            }
            let file_path = file_path.to_str().expect("msg").to_string();
            let real_filepath = fs::read_link(Path::new(&file_path));
            let real_filepath = match real_filepath {
                Ok(path) => path,
                Err(_) => {
                    continue;
                }
            };

            let file = File::new(&real_filepath.to_str().unwrap().to_string());
            result.push(file);
        }

        Ok(result)
    }

    pub fn get_mmaps(&self) -> Result<Vec<MemoryMap>,STError>{
        let file = format!("/proc/{}/maps",self.pid);
        let contents = fs::read_to_string(file);
        let contents = match contents {
            Ok(s) => s,
            Err(error) => {
                return Err(STError::from(Box::new(error)));
            }
        };
        
        let mut result: Vec<MemoryMap> = Vec::default();
        let lines = contents.split("\n");
        for line in lines {
            let line = line.split("                    ").collect::<Vec<&str>>();
            
            let others = line[0].trim();
            let pathname;
            if line.len() < 2 {
                pathname = "";
            } else {
                pathname = line[1].trim();
            }
            let others = others.split(" ").collect::<Vec<&str>>();
            if others.len() != 5 {
                continue;
            }
            let inode = others[4].parse::<i128>().unwrap_or(i128::min_value());
            let dev_num = others[3];
            let offset = i128::from_str_radix(others[2],16).unwrap_or(i128::min_value());
            let perms = others[1];
            let address = others[0].split("-").collect::<Vec<&str>>();
            assert!(address.len() < 3);
            let start_addr = i128::from_str_radix(address[0],16).unwrap_or(i128::min_value());
            let end_addr = i128::from_str_radix(address[1],16).unwrap_or(i128::min_value());

            let mmap = MemoryMap { 
                start_addr: start_addr as u64, 
                end_addr: end_addr as u64, 
                size: (start_addr - end_addr) as u64, 
                pathname: pathname.to_string(), 
                inode: inode as u64, 
                dev: dev_num.to_string(), 
                perm: perms.to_string(),
            };
            result.push(mmap);
        }
        Ok(result)
    }
}
