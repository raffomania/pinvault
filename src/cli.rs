use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Opt {
    List,
    Download {
        url: String,
    },
    Server
}
