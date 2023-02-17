use std::{process::Command, collections::HashMap};

use regex::Regex;

use super::System;

impl System {
    pub fn get_sysinfo() -> HashMap<String,String> {
        let mut res = HashMap::new();
        let output = Command::new("uname")
            .arg("--kernel-release")
            .output()
            .unwrap();
        let output = String::from_utf8_lossy(&output.stdout).to_string();
        let kernel_version = output.trim();
        let re = Regex::new(r"(\d+\.\d+\.\d+)\-(.*)").unwrap();
        let cap = re.captures(&kernel_version).unwrap();
        let version = cap.get(1).unwrap().as_str();
        let release_v = cap.get(2).unwrap().as_str().to_string();
        let lowercase = release_v.to_lowercase();
        res.insert("version".to_string(), version.to_string());
        if lowercase.contains("ubuntu") {
            res.insert("distro".to_string(), "ubuntu".to_string());
        }

        if lowercase.contains("debian") {
            res.insert("distro".to_string(), "debian".to_string());
        }

        if lowercase.contains("fedora") {
            res.insert("distro".to_string(), "fedora".to_string());
        }

        if lowercase.contains("centos") {
            res.insert("distro".to_string(), "centos".to_string());
        }

        if lowercase.contains("kali") {
            res.insert("distro".to_string(), "kali".to_string());
        }

        unimplemented!();

    }
}