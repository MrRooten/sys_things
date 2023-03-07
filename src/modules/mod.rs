use std::collections::HashMap;

use crate::utils::STError;

use self::{process::{CheckModifiedSectionV1, TestInjectCode, ProcessStrings, ProcessRefFiles}, privilege::CheckSystemVulns};



pub struct STResult {

}

pub type STArgs = HashMap<String,String>;
pub trait STModule {
    fn run(&self, args: &STArgs) -> Result<STResult,STError>;

    fn helper(&self) -> String;

    fn get_name(&self) -> String;

    fn get_detail(&self) -> String;
}

pub mod process;
pub mod autoruns;
pub mod privilege;
pub mod file;
pub mod system;
pub fn load_modules(vs: &mut Vec<Box<dyn STModule>>) {
    vs.push(Box::new(CheckModifiedSectionV1{}));
    vs.push(Box::new(TestInjectCode{}));
    vs.push(Box::new(ProcessStrings{}));
    vs.push(Box::new(CheckSystemVulns{}));
    vs.push(Box::new(ProcessRefFiles{}));
}