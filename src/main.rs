use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use url::Url;
use structopt::clap::AppSettings::*;

#[derive(StructOpt, Debug)]
#[structopt(setting(ColorAuto), setting(ColoredHelp), about = "reformat Novogene linklist.txt for Aria2c to keep dir structures")]
struct Opt {
    // The number of occurrences of the `v/verbose` flag
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    #[structopt(short, long)]
    create_dir: bool,

    #[structopt(
        short,
        long,
        name = "root",
        help = "which path segment to truncate to be root"
    )]
    root: Option<String>,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str), help = "files like 'linklist.txt'")]
    files: Vec<PathBuf>,
}

fn read_linklist(
    p: &PathBuf,
    root: &Option<String>,
    create_dir: bool,
) -> Result<(), Box<dyn Error>> {
    let s = fs::read_to_string(p)?;
    for line in s.lines() {
        if !line.starts_with("http") {
            continue;
        }
        let u = Url::parse(line).unwrap();
        let segs = u.path_segments().unwrap().collect::<Vec<_>>();
        let path = segs
            .into_iter()
            .skip_while(|&x| {
                if root.is_none() {
                    false
                } else {
                    root.as_ref().unwrap() != x
                }
            })
            .collect::<PathBuf>();
        let dir = path
            .parent()
            .expect(&format!("error path: {}", &path.display()));
        if create_dir && !dir.exists() {
            // std::fs::create_dir_all(dir)?;
            eprintln!("creating dir skipped: {:?}", dir);
        };
        println!("{}", &u);
        println!(" dir={}", dir.display());
        println!(" out={}", path.file_name().unwrap().to_str().unwrap());
    }
    Ok(())
}
fn main() {
    let args = Opt::from_args();
    eprintln!("{:?}", args);
    // let create = args.create_dir;
    // let root = args.root.clone();
    args.files.iter().for_each(|f| {
        read_linklist(&f, &args.root, args.create_dir).unwrap();
    });
}
