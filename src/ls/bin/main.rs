use std::{env, process, path::Path};
use std::fs::{read_dir, DirEntry, metadata};
use std::os::windows::prelude::*;
use colored::{Colorize, ColoredString};

type DirItems = Vec<Result<DirEntry, std::io::Error>>;

const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;

#[allow(dead_code)]
enum ListingFormat {
    Long,
    Commas,
    Horizontal,
    Vertical
}

struct UsedFlags {
    list_format: ListingFormat,
    show_hidden: bool,
}

impl Default for UsedFlags {
    fn default() -> Self {
        UsedFlags {
            list_format: ListingFormat::Horizontal,
            show_hidden: false,
        }
    }
}

fn usage() {
    println!("Enter directory name");
}

fn parse_params(params: &[String]) -> UsedFlags {
    let mut used_params: UsedFlags = Default::default();
    if params.len() > 2 {
        for param in params {
            match param.as_str() {
                "-l" => used_params.list_format = ListingFormat::Long,
                "-h" => used_params.show_hidden = true,
                _ => {},
            }
        }
    }
    used_params
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        usage();
        process::exit(1);
    }

    let flags = parse_params(&args);

    let dir_path = Path::new(&args[1]);

    if !dir_path.is_dir() {
        println!("{} is not a directory", dir_path.display());
        process::exit(1);
    }
    let entries: DirItems = read_dir(dir_path).unwrap().collect();

    match flags.list_format {
        ListingFormat::Long => long_print(&entries, &flags),
        ListingFormat::Commas => todo!(),
        ListingFormat::Horizontal => simple_print(&entries, &flags),
        ListingFormat::Vertical => todo!(),
    }
    
}

fn long_print(entries: &DirItems, flags: &UsedFlags) {
    for entry in entries {
        let item = entry.as_ref().unwrap();
        let file_meta = metadata(&item.path()).unwrap();
        let attributes = file_meta.file_attributes(); 
        if attributes & FILE_ATTRIBUTE_HIDDEN > 0 && !flags.show_hidden {
            continue;
        }
        let file_size = file_meta.file_size();
         
        let os_file_path = &item.file_name();
        let mut file_name: ColoredString = os_file_path.to_str().unwrap().into();
        if file_meta.is_dir() {
            file_name = file_name.purple();
        } else if file_meta.is_file() {
            file_name = file_name.green();
        } else {
            file_name = file_name.red();
        }

        let permissions = file_meta.permissions();
        let read_only = if permissions.readonly() {"Read-Only"} else {""}; 
        println!("{:>10} {:>10} {:>20}", &read_only, &file_size, &file_name);
    }
}

fn simple_print(entries: &DirItems, flags: &UsedFlags) {
    let mut count: u32 = 0;
    for entry in entries {
        let item = entry.as_ref().unwrap();
        let file_meta = metadata(&item.path()).unwrap();
        let attributes = file_meta.file_attributes(); 
        if attributes & FILE_ATTRIBUTE_HIDDEN > 0 && !flags.show_hidden {
            continue;
        }
         
        if count != 0 { print!(" "); }
        let os_file_path = &item.file_name();
        let mut file_name: ColoredString = os_file_path.to_str().unwrap().into();
        if file_meta.is_dir() {
            file_name = file_name.purple();
        } else if file_meta.is_file() {
            file_name = file_name.green();
        } else {
            file_name = file_name.red();
        }

        print!("{}", &file_name);
        count += 1;
    }
}
