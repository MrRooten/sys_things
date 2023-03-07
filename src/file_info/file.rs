use std::{path::{Path, PathBuf}, fs, default, ffi::CString, alloc::System, mem, error::Error};

use chrono::{NaiveDateTime, DateTime, Utc};
use libc::{fstat, stat, lstat, malloc, free, c_void};
use posix_acl::{PosixACL, ACLError};



use crate::{account_info::{User, Group}, utils::func::i_to_m};

use super::File;

impl File {
    pub fn new(r_path : &String) -> Self{
        File {
            r_path : r_path.to_string(),
            ..Default::default()
        }
    }

    pub fn procs_open_this(&self) -> Result<Vec<u32>,Box<dyn Error>> {
        let my_path = Path::new(&self.r_path);
        let path = Path::new("/proc");
        let mut pids: Vec<u32> = Vec::default();
        let read_dir = fs::read_dir(path);
        let read_dir = match read_dir {
            Ok(dir) => dir,
            Err(err) => {
                return Err(Box::new(err));
            }
        };
        for entry in read_dir {
            let entry = match entry {
                Ok(entry) => entry,
                Err(err) => {
                    continue;
                }
            };
            //println!("{:?}", entry.file_name());
            let s = entry.file_name().into_string();
            let s = match s {
                Ok(file) => file,
                Err(error) => {
                    error.to_str().unwrap().to_string()
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
                let real_filepath = real_filepath.expect("msg");
                if real_filepath.eq(my_path) {
                    pids.push(pid);
                }
            }
        }
        Ok(pids)
    }
    
    fn set_stat(&mut self) {
        unsafe {
            let c_str = CString::new(self.r_path.to_string()).expect("");
            let mut arr = [0 as u8;mem::size_of::<stat>()];
            let mut s_stat = arr.as_ptr();
            lstat(c_str.as_ptr(),s_stat as *mut stat);
            self.r_stat = Some(*(s_stat as *mut stat));
            self.stat_is_set = true;
        }
    }

    pub fn get_modify_time(&mut self) -> DateTime<Utc> {
        self.set_stat();
        let modify_time = self.r_stat.unwrap().st_mtime;
        let native = NaiveDateTime::from_timestamp_opt(modify_time, 0).unwrap();
        let datetime : DateTime<Utc> = DateTime::from_utc(native,Utc);
        datetime
    }

    pub fn get_change_time(&self) {

    }

    pub fn get_create_time(&mut self) -> DateTime<Utc> {
        self.set_stat();
        let modify_time = self.r_stat.unwrap().st_ctime;
        let native = NaiveDateTime::from_timestamp_opt(modify_time, 0).unwrap();
        let datetime : DateTime<Utc> = DateTime::from_utc(native,Utc);
        datetime
    }

    pub fn get_access_time(&self) -> DateTime<Utc>{
        let self2 = i_to_m(self);
        self2.set_stat();
        let modify_time = self2.r_stat.unwrap().st_atime;
        let native = NaiveDateTime::from_timestamp_opt(modify_time, 0).unwrap();
        let datetime : DateTime<Utc> = DateTime::from_utc(native,Utc);
        datetime
    }

    pub fn get_filesize(&self) -> i64{
        let self2 = i_to_m(self);
        self2.set_stat();
        self.r_stat.unwrap().st_size
    }

    pub fn get_filetype(&self) {

    }

    pub fn get_filepath(&self) -> String {
        match fs::canonicalize(&self.r_path) {
            Ok(o) => {
                return o.to_string_lossy().to_string();
            },
            Err(e) => {
                return format!("[deleted]{}", self.r_path);
            }
        }
    }

    pub fn get_inode(&self) -> u64{
        let self2 = i_to_m(self);
        self2.set_stat();
        self.r_stat.unwrap().st_ino
    }

    pub fn get_priv(&self) {

    }

    pub fn get_acls(&self) -> Result<PosixACL, ACLError>{
        PosixACL::read_acl(self.r_path.to_string())
    }

    pub fn get_owner(&mut self) -> User {
        self.set_stat();
        User::new(self.r_stat.unwrap().st_uid)
    }

    pub fn get_group(&self) -> Group {
        unimplemented!("")
    }

    pub fn get_n_hardlink(&self) {

    }

}