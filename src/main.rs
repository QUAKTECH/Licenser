use std::env;
use std::fs;
use std::process;

mod array;

fn display_usage() {
    println!("Usage: licenser <Name of license> <File name>");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        display_usage();
        process::exit(1);
    }

    let templates = array::get_templates();

    let arg1 = &args[1];
    let arg2 = &args[2];

    // Convert arg1 to a String before using it as a key
    if let Some(template_file) = templates.get(arg1.as_str()) {
        if template_file.exists() {
            if let Err(e) = fs::copy(template_file, arg2) {
                eprintln!("Failed to copy {} to {}: {}", template_file.display(), arg2, e);
                process::exit(1);
            }
            println!("{} has been copied to {}.", arg1, arg2);
        } else {
            eprintln!("Template file {} does not exist.", template_file.display());
            process::exit(1);
        }
    } else {
        eprintln!("The specified license name ({}) is not supported.", arg1);
        process::exit(1);
    }
}