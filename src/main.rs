use std::path::{Path};
use std::ffi::OsStr;
use std::process::exit;
use std::iter::{FromIterator};
use ansi_term::{Colour, ANSIGenericStrings};
use clap::ArgMatches;
use walkdir::{WalkDir, DirEntry};
use std::borrow::Borrow;

#[macro_use]
extern crate clap;


const BRANCH_SPLIT: char = '├';
const BRANCH_CONTINUATION: char = '│';
const ROOT_DIR: char = '┌';
const FILE_ICON: char = '';
const DIR_ICON: char = '';
const SPACE: char = ' ';

fn main() {
    let matches: ArgMatches = clap_app!(myapp =>
        (version: "0.1")
        (author: "github.com/tjweldon")
        (about: "A nicer looking version of unix tree")
        (@arg DEPTH: -d --depth +takes_value "Sets the how deep into the directory structure the tree will recurse")
        (@arg ALL: -a --all "Shows hidden files/folders")
        (@arg workingdir: "Specifies the root folder to produce a tree from, defaults")
    ).get_matches();
    let working_dir = matches.value_of("workingdir").unwrap_or(".");
    let recursion_depth: i32 = matches.value_of("DEPTH").unwrap_or("-1").parse().unwrap_or(-1);
    let show_all: bool = matches.is_present("ALL");

    let absolute_path = match to_abs(working_dir) {
        Some(s) => s,
        None => {
            println!("Path failed to parse");
            exit(1)
        }
    };

    let walkdir = get_walkdir(&absolute_path);

    let walkdir = apply_recursion_depth(recursion_depth, walkdir);


    print_root_dir(Path::new(&absolute_path));
    for (_i, entry) in walkdir.into_iter().enumerate() {
        // if _i > 10 { exit(0) }
        match to_rel(&entry.path(), &Path::new(&absolute_path)) {
            Ok(p) => print_path(p, show_all),
            // Ok(p) => println!("{}{:?}", p.to_str().unwrap(), p.is_dir()),
            Err(_) => ()
        };
    }
}

fn apply_recursion_depth(recursion_depth: i32, full_depth: WalkDir) -> Vec<DirEntry> {
    let ok_paths = match recursion_depth >= 0 {
        true => full_depth.max_depth((recursion_depth + 1) as usize),
        false => full_depth
    }
        .into_iter()
        .filter_map(|e| e.ok())
        .into_iter()
        ;
    Vec::from_iter(ok_paths)
}

fn get_walkdir(absolute_path: &String) -> WalkDir {
    let full_depth = walkdir::WalkDir::new(&absolute_path)
        .sort_by(|a, b| a.file_name().cmp(b.file_name()))
        .min_depth(1).follow_links(true)
        ;
    full_depth
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

fn to_rel<'a>(abs_path: &'a Path, root: &'a Path) -> Result<&'a Path, &'a str> {
    let rel_path = Path::new(abs_path.to_str().unwrap());
    let result = match abs_path.starts_with(&root) {
        true => Ok(rel_path.strip_prefix(&root).unwrap()),
        false => Err("not a child path")
    };

    result
}

fn print_root_dir(path: &Path) -> () {
    let line = String::from_iter(
        [
            vec![DIR_ICON, SPACE],
            Vec::from_iter(path.to_str().unwrap().chars())
        ].concat()
    );

    println!("{}{}", ROOT_DIR, Colour::Green.paint(line))
}

fn print_path(rel_path: &Path, show_all: bool) -> () {
    if !show_all {
        let path_str = rel_path.borrow().to_str().unwrap();
        let is_hidden = path_str.contains("/.") || path_str.starts_with(".");
        if is_hidden {return;}
    }
    let depth = match rel_path.components().count() {
        0 => 0,
        x => x - 1
    };
    let prefix_vec = vec![BRANCH_CONTINUATION; depth];
    let painted = prefix_vec
        .into_iter()
        .enumerate()
        .map(
            |(i, delim)| {
                get_colour_from_palette(i).paint(format!("{}", delim))
            }
        );
    // let prefix = String::from_iter(prefix_vec.into_iter());
    let is_dir = match rel_path.is_dir() {
        true => true,
        false => match rel_path.symlink_metadata() {
            Ok(m) => m.file_type().is_dir(),
            Err(_) => false
        }
    };
    // println!("-------------------{:?}", is_dir);
    let last_delims = match rel_path.is_dir() {
        true => String::from_iter([BRANCH_SPLIT]),
        false => String::from_iter([BRANCH_CONTINUATION])
    };
    let full_prefix_vec = [
        Vec::from_iter(painted),
        Vec::from_iter([get_colour_from_palette(depth).paint(&last_delims)])
    ].concat();
    let full_prefix = ANSIGenericStrings(&full_prefix_vec);

    let icon = match is_dir {
        true => String::from_iter([DIR_ICON, SPACE]),
        false => String::from_iter([FILE_ICON, SPACE])
    };
    let suffix = match is_dir {
        true => "/",
        false => ""
    };
    // let full_prefix: String = [prefix, last_delims].concat() as String;

    let entry_name = match rel_path.file_name() {
        Some(s) => s.to_str().unwrap(),
        None => match rel_path.parent() {
            Some(p) => p.file_name().unwrap().to_str().unwrap(),
            None => ""
        }
    };

    let bar = [
        match is_dir {
            true => Colour::Cyan,
            false => Colour::Yellow
        }.paint(icon),
        Colour::White.paint(entry_name.to_string()),
        Colour::White.paint(suffix)
    ];
    let coloured_line = [
        full_prefix,
        ANSIGenericStrings(&bar)
    ];

    println!("{}{}", coloured_line[0], coloured_line[1])
}

fn ansi_grid_colour(grid_index: u8, grid_y: u8, grid_x: u8) -> u8 {
    16 + (grid_index % 6) * 36 + (grid_y % 6) * 6 + (grid_x % 6)
}

fn palette(index: u8) -> u8 {
    let grid_index = 5 - (index % 6) as u8;
    let grid_y = 5 - (index / 2 % 6) as u8;
    let grid_x = 4 as u8;
    let ansi_index = ansi_grid_colour(grid_index, grid_y, grid_x);
    ansi_index
}

fn get_colour_from_palette(i: usize) -> Colour {
    let col_index = palette(i as u8);
    let colour = Colour::Fixed(col_index);
    colour
}
