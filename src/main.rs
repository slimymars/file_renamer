use file_format::FileFormat;
use getopts::{self, Options};
use std::env;

fn file_rename(filename: &str, dryrun: bool) -> Result<String, std::io::Error>{
    let format = FileFormat::from_file(filename)?;
    let filename_ext_index = filename.rfind('.');
    let filename_ext = match filename_ext_index {
        None => "",
        Some(i) => &filename[i+1..],
    };
    let format_ext = format.extension();

    if filename_ext == format_ext {
        return Ok(format!("Need not: {}", filename).into());
    }
    let rename = match filename_ext_index {
        None => format!("{}.{}", filename, format_ext),
        Some(i) => format!("{}.{}", &filename[..i], format_ext),
    };
    if dryrun {
        return Ok(format!("Rename: {} -> {}", filename, rename).into());
    };
    std::fs::rename(filename, &rename)?;
    return Ok(format!("Rename: {} -> {}", filename, rename).into());
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] Files..", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let mut dryrun = false;
    let mut opts = getopts::Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("d", "dryrun", "dryrun mode");
    let args: Vec<String> = env::args().collect();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => { panic!("{}", f) }
    };
    if matches.opt_present("h") || matches.free.is_empty() {
        print_usage(&args[0], opts);
        return;
    }
    if matches.opt_present("d") {
        println!("{}", "!!! Dryrun Mode !!!");
        dryrun = true;
    }
    for filename in &matches.free {
        match file_rename(filename, dryrun) {
            Ok(log) => println!("{}", log),
            Err(e) => println!("{}: {}", filename, e),
        }
    }
}
