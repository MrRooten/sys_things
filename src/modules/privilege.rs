use colored::Colorize;
use regex::Regex;
use std::process::Command;

use crate::utils::STError;

use super::{STArgs, STModule, STResult};

pub struct CheckSystemVulns {}

fn is_small(my_version: &str,target_version: &str) -> bool {
    let re = Regex::new(r"(\d+)\.(\d+)\.(\d+)").unwrap();
    let my = re.captures(my_version).unwrap();
    let target = re.captures(target_version).unwrap();
    let big = my.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let middle = my.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let small = my.get(3).unwrap().as_str().parse::<i32>().unwrap();

    let t_big = target.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let t_middle = target.get(2).unwrap().as_str().parse::<i32>().unwrap();
    let t_small = target.get(3).unwrap().as_str().parse::<i32>().unwrap();

    if big < t_big {
        return true;
    } else if big == t_big {
        if middle < t_middle {
            return true;
        } else if middle == t_middle {
            if small < t_small {
                return true;
            }
        } else {
            return false;
        }
    }
    return false;
}

impl STModule for CheckSystemVulns {
    fn run(&self, args: &STArgs) -> Result<STResult, STError> {
        let output = Command::new("uname")
            .arg("--kernel-release")
            .output()
            .unwrap();
        let output = String::from_utf8_lossy(&output.stdout).to_string();
        let kernel_version = output.trim();
        let re = Regex::new(r"(\d+\.\d+\.\d+(.\d+)?)\-(.*)").unwrap();
        let cap = re.captures(&kernel_version).unwrap();
        let version = cap.get(1).unwrap().as_str();
        let mut release_v = String::default();
        if cap.len() == 4 {
            release_v = cap.get(3).unwrap().as_str().to_string();
        } else {
            release_v = cap.get(2).unwrap().as_str().to_string();
        }
        
        println!("Kernel version:{}", version.green());
        println!("Release version:{}",release_v.green());
        println!("Checking CVE-2016-5195...");
        if version.starts_with("2") {
            println!(
                "[{}] for dirty cow [{}]",
                "vuln".red(),
                "CVE-2016-5195".blue()
            );
        }

        if version.starts_with("3") {
            if release_v.to_lowercase().contains("centos") {
                if is_small(version, "3.327.36") {
                    println!(
                        "[{}] for dirty cow [{}]",
                        "vuln".red(),
                        "CVE-2016-5195".blue()
                    );
                }
            }

            if release_v.to_lowercase().contains("ubuntu") {
                if release_v.contains("12.04") {
                    if is_small(version, "3.113.155") {
                        println!(
                            "[{}] for dirty cow [{}]",
                            "vuln".red(),
                            "CVE-2016-5195".blue()
                        );
                    }
                }
                if release_v.contains("14.04") {
                    if is_small(version, "3.100.147") {
                        println!(
                            "[{}] for dirty cow [{}]",
                            "vuln".red(),
                            "CVE-2016-5195".blue()
                        );
                    }
                }
            }
        }

        if version.starts_with("4") {
            if release_v.to_lowercase().contains("ubuntu") {
                if release_v.contains("16.04") {
                    if is_small(version, "4.45.66") {
                        println!(
                            "[{}] for dirty cow [{}]",
                            "vuln".red(),
                            "CVE-2016-5195".blue()
                        );
                    }
                }

                if release_v.contains("16.10") {
                    if is_small(version, "4.26.28") {
                        println!(
                            "[{}] for dirty cow [{}]",
                            "vuln".red(),
                            "CVE-2016-5195".blue()
                        );
                    }
                }
            }
        }
        println!("Checking CVE-2017-0358...");
        if release_v.to_lowercase().contains("ubuntu") && (release_v.contains("16.04") || release_v.contains("16.10")) {
            println!(
                "[{}] Debian/Ubuntu ntfs-3g Local Privilege Escalation [{}]",
                "vuln".red(),
                "CVE-2017-0358".blue()
            );
        }

        if release_v.to_lowercase().contains("debian") && (release_v.contains("7") || release_v.contains("8")) {
            println!(
                "[{}] Debian/Ubuntu ntfs-3g Local Privilege Escalation [{}]",
                "vuln".red(),
                "CVE-2017-0358".blue()
            );
        }
        println!("Checking CVE-2018-18955...");
        println!("Checking CVE-2018-1000001...");
        println!("Checking CVE-2021-3560...");
        let output = Command::new("pkexec version")
            .arg("--version")
            .output();
        loop {
            let output = match output {
                Ok(o) => o,
                Err(e) => {
                    break;
                }
            };
            let output = String::from_utf8_lossy(&output.stdout).to_string();
            let regex = Regex::new(r"pkexec version (\d+\S*)").unwrap();
            let cap = regex.captures(&output);
            if cap.is_none() {
                break;
            }

            let cap = cap.unwrap();
            break;
        }


        Ok(STResult {})
    }

    fn helper(&self) -> String {
        todo!()
    }

    fn get_name(&self) -> String {
        "CheckSystemVulns".to_string()
    }

    fn get_detail(&self) -> String {
        todo!()
    }
}
