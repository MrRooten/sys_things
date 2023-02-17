use super::STModule;

pub struct AutoRuns {

}

impl STModule for AutoRuns {
    fn run(&self, args: &super::STArgs) -> Result<super::STResult,crate::utils::STError> {
        let rcs = "/etc/rc{}.d";
        let crondir_files = "/etc/cron.{}";
        let crontab_file = "/etc/crontab";
        let sh_rc = "/home/{}/{}";
        let profile = "/etc/profile";
        let profiles = "/etc/profile.d/{}";
        unimplemented!()
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