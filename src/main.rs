use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "porgy",
    about = "A CLI to genrate Prost rust code from protobuf."
)]
struct Opt {
    #[structopt(long = "include", short = "I", parse(from_os_str))]
    includes: Option<Vec<PathBuf>>,

    #[structopt(required(true), parse(from_os_str))]
    protos: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
