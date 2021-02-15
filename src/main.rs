use std::env;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "porgy",
    about = "A CLI to genrate Prost rust code from protobuf."
)]
struct Opt {
    #[structopt(
        name = "OUTPUT_DIR",
        env = "OUT_DIR",
        long = "output",
        short = "o",
        default_value = "."
    )]
    output: String,

    #[structopt(
        name = "INCLUDE_PATH",
        long = "include",
        short = "I",
        require_delimiter(true),
        parse(from_os_str)
    )]
    includes: Vec<PathBuf>,

    #[structopt(required(true), parse(from_os_str))]
    protos: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    let out_dir = "OUT_DIR";

    let old_envvar = env::var(out_dir);
    env::set_var(out_dir, opt.output);

    prost_build::compile_protos(&opt.protos, &opt.includes)?;

    if let Ok(val) = old_envvar {
        env::set_var(out_dir, val);
    } else {
        env::remove_var(out_dir);
    }
    Ok(())
}
