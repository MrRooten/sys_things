use std::{path::{Path, PathBuf}, fs};

use crate::{process_info::Process, file_info::File};

use super::User;


impl User {
    pub fn new(uid : u32) -> Self{
        User {
            uid : uid
        }
    }

    pub fn get_userid(&self) -> u32{
        self.uid
    }

    pub fn get_username(&self) -> String {
        String::default()
    }

    pub fn get_processes(&self) -> Vec<Process> {
        Vec::default()
    }

    pub fn get_open_files(&self) -> Vec<File> {
        let path = Path::new("/proc");
        let mut result: Vec<File> = Vec::default();
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

            let status_file = format!("/proc/{}/status",pid);
            let status = fs::read_to_string(status_file);
            let status = match status {
                Ok(s) => s,
                Err(e) => {
                    continue;
                }
            };
            let mut uid_line = "";
            for line in status.split("\n") {
                if line.starts_with("Uid") {
                    uid_line = line;
                    break;
                }
            }

            let pair = uid_line.split(":");
            let pair = pair.collect::<Vec<&str>>();
            let key = pair[0];
            let value = pair[1];

            let uids = value.trim().split("\t").collect::<Vec<&str>>();
            for uid in uids {
                let uid = uid.parse::<u32>().expect("Not a valid uid");
                if (uid == self.uid) {
                    let fd_dir = format!("/proc/{}/fd/",pid);
                    let fds = fs::read_dir(fd_dir);
                    let fds = match fds {
                        Ok(_fds) => _fds,
                        Err(error) => {
                            continue;
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
                        let i_fd = fd.file_name().to_str().expect("Not a valid fd").parse::<i32>().expect("Not a valid number");
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
                } 
            }
        }
        result
    }

    pub fn get_login_time(&self) {

    }

    pub fn get_caps(&self) {

    }

    pub fn get_create_time(&self) {

    }

    pub fn list_all_users() -> Vec<User> {
        unimplemented!("")
    }

    pub fn current_login() -> Vec<User> {
        unimplemented!("")
    }

    pub fn change_home(&self, dir: PathBuf) {

    }

    pub fn add_sudo_priv(&self) {

    }

    pub fn change_expiredate(&self) {

    }

    pub fn change_password(&self, password: String) {

    }

    pub fn change_shell(&self, shell: PathBuf) {

    }

    pub fn change_uid(&self, uid: u32) {

    }
}