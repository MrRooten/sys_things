use std::{fs, os::unix::prelude::FileTypeExt};

use chrono::ParseError;
use elf::Section;

use crate::utils::STError;


pub struct ELFParser {
    file    : elf::File
}

pub struct Type(pub u16);
/// No file type
pub const ET_NONE : Type = Type(0);
/// Relocatable object file
pub const ET_REL : Type = Type(1);
/// Executable file
pub const ET_EXEC : Type = Type(2);
/// Shared library
pub const ET_DYN : Type = Type(3);
/// Core file
pub const ET_CORE : Type = Type(4);

impl ELFParser {
    pub fn new(file: &str) -> Result<Self,STError>{
        if file.starts_with("/dev") {
            return Err(STError::new("Can not open dev file"));
        }

        let metadata = fs::metadata(file);
        let metadata = match metadata {
            Ok(data) => data,
            Err(e) => {
                let err_s = format!("{:?}",e);
                return Err(STError::new(&err_s));
            }
        };

        if metadata.file_type().is_fifo() {
            return Err(STError::new("Can be fifo"));
        }
        let file = elf::File::open_path(file);
        let ret = match file {
            Ok(f) => f,
            Err(err) => {
                let err_s = format!("{:?}",err);
                return Err(STError::new(&err_s));
            }
        };
        let parser = ELFParser {
            file    : ret
        };
        Ok(parser)
    }

    pub fn get_segments(&self) -> &Vec<Section>{
        &self.file.sections
    }

    pub fn get_type(&self) -> Type {
        Type(self.file.ehdr.elftype.0)
    }
}