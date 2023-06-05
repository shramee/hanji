use clap::Parser;
use hanji::utils::get_cairo_files_in_path;
use hanji::{run_printer, MarkdownEngine, TemplateEngine};

use std::fs::{create_dir_all, remove_dir_all, remove_file, File};
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "hanji", author, version)]
/// Welcome to Hanji, Hanji builds docs for your Cairo code.
struct Cli {
    /// Path to the cairo file or directory to parse
    path: PathBuf,

    /// Path to output the docs in, default hanji-out
    out_dir: Option<PathBuf>,

    /// Print the index, can be pasted in readme.md
    #[arg(short, long)]
    index: bool,

    /// Index links path prefix, defaults to out_dir path
    #[arg(short = 'x', long)]
    index_path_prefix: Option<PathBuf>,
}

fn main() {
    let mut cli = Cli::parse();

    let out_path = match cli.out_dir.clone() {
        Some(path) => path,
        None => PathBuf::from("hanji-out"),
    };

    // Clean out_path
    if out_path.exists() {
        match remove_dir_all(&out_path) {
            Err(_) => {
                println!("Clearing out_path file {:?}", out_path);
                remove_file(&out_path).unwrap();
            }
            _ => {}
        }
    }

    let mut index = String::new();
    cli.index_path_prefix = match cli.index_path_prefix {
        Some(path) => Some(path),
        None => Some(out_path.clone()),
    };

    if cli.path.is_dir() {
        create_dir_all(&out_path).unwrap();
        let cairo_files = get_cairo_files_in_path(&cli.path);
        for cairo_file in cairo_files.iter() {
            handle_md_file(cairo_file, &out_path, &mut index, &cli);
        }
    } else {
        handle_md_file(&cli.path, &out_path, &mut index, &cli);
    }

    if cli.index {
        print!("{}", index);
    }
}

fn handle_md_file(cairo_file: &PathBuf, out_path: &PathBuf, index: &mut String, cli: &Cli) {
    let rel_path = cairo_file.strip_prefix(&cli.path).unwrap();
    let mut doc_file_path = out_path.clone();

    if doc_file_path.is_dir() {
        doc_file_path.push(rel_path);
    }
    doc_file_path.set_extension("md");

    create_dir_all(&doc_file_path.parent().unwrap()).unwrap();

    let mut file = File::create(&doc_file_path.into_os_string().to_str().unwrap()).unwrap();

    let engine = run_printer(cairo_file.to_str().unwrap(), MarkdownEngine::new()).unwrap();

    for (fn_head, fn_doc) in &engine.fn_index {
        index.push_str(&format!(
            "- [`{}`]({}/{}#{}) {}\n",
            fn_head.replace("Function", "fn"),
            cli.index_path_prefix.clone().unwrap().to_str().unwrap(),
            rel_path.with_extension("md").to_str().unwrap(),
            fn_head.to_lowercase().replace(" ", "-"),
            fn_doc
        ));
    }

    file.write_all(engine.get_result().as_bytes()).unwrap();
}
