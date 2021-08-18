use std::env;
use std::path::Path;
use std::ffi::OsStr;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    let working_dir = match args.len() {
        2 => &args[1],
        _ => "."
    };

    let absolute_path = match to_abs(working_dir) {
        Some(s) => s,
        None => {
            println!("Path failed to parse");
            exit(1)
        }
    };

    let ok_paths = walkdir::WalkDir::new(Path::new(&absolute_path))
        .into_iter()
        .filter_map(|e| e.ok())
    ;

    for (i, entry) in ok_paths.enumerate() {
        if i > 10 { exit(0) }
        println!("{}", entry.path().display());
    }
}

fn to_abs(p: &str) -> Option<String> {
    shellexpand::full(p)
        .ok()
        .and_then(
            |x| Path::new(OsStr::new(x.as_ref())).canonicalize().ok()
        ).and_then(
            |y| y.into_os_string().into_string().ok()
    )
}
