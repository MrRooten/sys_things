use std::fs;
use std::path::Path;
#[derive(Default)]
pub struct Process {
    pid  : u32,
    name : String,
    cmdline : String,
    memory : Memory
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

pub struct User {

}


impl Process {
    pub fn list_process_in_proc() {
        
        let path = Path::new("/proc");
        for entry in fs::read_dir(path).expect("Unable to list") {
            let entry = entry.expect("unable to get entry");
            //println!("{:?}", entry.file_name());
            let s = entry.file_name().into_string();
            let s = match s {
                Ok(file) => file,
                Err(error) => {
                    panic!("Problem opening the file: {:?}", error)
                },
            };

            let s1 = String::from(s);
            let test = s1.parse::<u32>();
            let pid = match test {
                Ok(ok) => ok,
                Err(e) => {
                    continue;
                }
            };

            println!("{:?}",pid);
        }
    }

    pub fn new(pid : u32) -> Process {
        Process {
            pid: pid,
            ..Default::default()
        }
    }

    pub fn get_name(&mut self) -> &String{
        let comm_file = format!("/proc/{}/comm",self.pid);
        let contents = fs::read_to_string(comm_file);
        let contents = match contents {
            Ok(file) => file,
            Err(_e) => "".to_string()
        };
        self.name = contents;
        &self.name
    }

    pub fn get_cmdline(&mut self) -> &String{
        let comm_file = format!("/proc/{}/cmdline",self.pid);
        let contents = fs::read_to_string(comm_file);
        let contents = match contents {
            Ok(file) => file,
            Err(_e) => "".to_string()
        };
        self.cmdline = contents;
        &self.cmdline
    }
    fn get_stat(&self,key : &String) -> String {
        let stat_file = format!("/proc/{}/stat",self.pid);
        let contents = fs::read_to_string(stat_file);
        let contents = match contents {
            Ok(file) => file,
            Err(_e) => _e.to_string()
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
        }  else if key.eq("signal") {
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

    fn get_status(&self,key : &String) -> String {
        let status_file = format!("/proc/{}/comm",self.pid);
        let contents = fs::read_to_string(status_file);
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
            if (key.to_string().trim().eq(key)) {
                return value.trim().to_string();
            }
        }
        "".to_string()
    }

    pub fn get_state(&mut self) -> String {
        self.get_stat(&"state".to_string())
    }

    pub fn get_memory(&mut self) -> &Memory {
        let status_file = format!("/proc/{}/comm",self.pid);
        let contents = fs::read_to_string(status_file);
        let contents = match contents {
            Ok(file) => file,
            Err(_e) => "".to_string()
        };

        let lines = contents.split("\n");
        for line in lines {
            let item = line.split(":");
            let key_value = item.collect::<Vec<&str>>();
            let key = key_value[0].trim().to_string();
            let value = key_value[1].trim().to_string();
            if (key.eq("VmPeak")) {
                
            }
        }
        &self.memory
    }
}
