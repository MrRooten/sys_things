#![allow(unused)]

use std::{env, collections::HashMap, vec};
use sys_things::modules::load_modules;
use sys_things::modules::privilege::CheckSystemVulns;
use sys_things::modules::process::ProcessStrings;
use sys_things::{process_info::Process, file_info::elf::ELFParser, modules::{process::{CheckModifiedSectionV1}, STModule, STArgs}};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("{} help",args[0]);
        println!("{} run {} {}",args[0],"${ModuleName}","key=value...");
        println!("{} list",args[0]);
    }

    let mut v1: Vec<Box<dyn STModule>> = vec![];
    load_modules(&mut v1);
    if args[1].trim().eq("run") {
        let len_args = args.len();
        let mut index = 3;
        let mut st_args = STArgs::new();
        while index < len_args {
            assert!(index < len_args);
            let kv = args[index].split("=").collect::<Vec<&str>>();
            if kv.len() < 2 {
                println!("warning:{} is not valid",args[index]);
                continue;
            }
            st_args.insert(kv[0].to_string(),kv[1].to_string());
            index += 1;
        }
        for m in v1 {
            if args[2].eq(&m.get_name()) {
                m.run(&st_args);
            }
        }
    } else if args[1].trim().eq("test") {
        
    } else if args[1].trim().eq("list") {
        let len_args = args.len();
        let mut index = 3;
        let mut st_args = STArgs::new();
        for m in v1 {
            println!("{}",m.get_name());
        }
    }
}
