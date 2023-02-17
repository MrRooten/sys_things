use crate::utils::STError;

use super::{STModule, STArgs, STResult};

pub struct ReasonForShutdown {

}

impl STModule for ReasonForShutdown {
    fn run(&self, args: &STArgs) -> Result<STResult,STError> {
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