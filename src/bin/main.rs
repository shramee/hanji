use clap::Parser;
use hanji::print_markdown;
use hanji::utils::get_cairo_files_in_path;

use std::fs::{create_dir_all, remove_dir_all, remove_file, File};
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to the cairo file or directory to parse
    path: String,

    /// The directory to output the docs in
    out_dir: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let in_path = PathBuf::from(cli.path);

    let out_path = match cli.out_dir {
        Some(path) => PathBuf::from(&path),
        None => PathBuf::from("hanji-out"),
    };

    // Clean out_path
    if out_path.exists() {
        println!("Clearing out_path {:?}", out_path);
        match remove_dir_all(&out_path) {
            Err(_) => {
                println!("Clearing out_path file {:?}", out_path);
                remove_file(&out_path).unwrap();
            }
            _ => {}
        }
    }

    if in_path.is_dir() {
        create_dir_all(&out_path).unwrap();
        let cairo_files = get_cairo_files_in_path(&in_path);
        for cairo_file in cairo_files.iter() {
            handle_md_file(cairo_file, &in_path, &out_path);
        }
    } else {
        handle_md_file(&in_path, &in_path, &out_path);
    }
}

fn handle_md_file(cairo_file: &PathBuf, in_path: &PathBuf, out_path: &PathBuf) {
    let rel_path = cairo_file.strip_prefix(in_path).unwrap();
    let mut doc_file_path = out_path.clone();

    if doc_file_path.is_dir() {
        doc_file_path.push(rel_path);
    }
    doc_file_path.set_extension("md");

    create_dir_all(&doc_file_path.parent().unwrap()).unwrap();

    let mut file = File::create(&doc_file_path.into_os_string().to_str().unwrap()).unwrap();

    let docs = print_markdown(cairo_file.to_str().unwrap());
    file.write_all(docs.as_bytes()).unwrap();
}
