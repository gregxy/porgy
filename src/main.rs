use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "porgy",
    about = "A CLI to genrate Prost rust code from protobuf."
)]
struct Opt {
    #[structopt(name = "INCLUDE_PATH", long = "include", short = "I",
        require_delimiter(true), parse(from_os_str))]
    includes: Vec<PathBuf>,

    #[structopt(required(true), parse(from_os_str))]
    protos: Vec<PathBuf>,
}

fn main() {
    let opt = Opt::from_args();
    prost_build::compile_protos(&opt.protos, &opt.includes).unwrap();
}
