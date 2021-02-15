use std::collections::HashSet;
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
    let mut opt = Opt::from_args();

    let mut addl_includes: HashSet<PathBuf> = HashSet::new();
    addl_includes.insert(PathBuf::from("."));
    for p in opt.protos.iter() {
        if let Some(parent) = p.parent() {
            if parent.is_dir() {
                addl_includes.insert(parent.to_path_buf());
            }
        }
    }

    for p in addl_includes.iter() {
        opt.includes.push(p.clone());
    }

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
