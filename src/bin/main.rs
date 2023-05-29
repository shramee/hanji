use hanji::print_markdown;
use hanji::utils::get_cairo_files_in_path;
use std::env;
use std::fs::{create_dir, create_dir_all, remove_dir_all, File};
use std::io::prelude::*;
use std::path::PathBuf;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("File path to parse is required.")
    }

    let in_path = PathBuf::from(&args[1]);

    let out_path = match args.get(2) {
        Some(path) => PathBuf::from(&path),
        None => PathBuf::from("hanji-out"),
    };

    if in_path.is_dir() {
        remove_dir_all(&out_path).unwrap();
        create_dir(&out_path).unwrap();
        let cairo_files = get_cairo_files_in_path(&in_path);
        for cairo_file in cairo_files.iter() {
            let rel_path = cairo_file.strip_prefix(&in_path).unwrap();
            let mut doc_file_path = out_path.clone();
            doc_file_path.push(rel_path);
            doc_file_path.set_extension("md");

            create_dir_all(&doc_file_path.parent().unwrap()).unwrap();

            let mut file = File::create(&doc_file_path.into_os_string().to_str().unwrap()).unwrap();

            let docs = print_markdown(cairo_file.to_str().unwrap());
            file.write_all(docs.as_bytes()).unwrap();
        }
    } else {
        print_cairo_file_docs(&args[1]);
    }

    // Cairo language spec
    // cairo_lang_syntax_codegen::cairo_spec::get_spec
}

fn print_cairo_file_docs(cairo_filename: &str) {
    println!("{}", print_markdown(cairo_filename));
}
