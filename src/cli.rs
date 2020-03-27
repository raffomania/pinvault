use clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    pub enum Command {
        Download,
        List
    }
}

#[derive(StructOpt, Debug)]
pub enum Opt {
    List,
    Download {
        url: String,
    }
}
