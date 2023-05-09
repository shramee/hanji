pub mod printer;
pub mod template_engine;
use std::env;

use cairo_lang_parser::utils::{get_syntax_root_and_diagnostics_from_file, SimpleParserDatabase};
use printer::run_printer;
use template_engine::MarkdownEngine;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("File path to parse is required.")
    }
    generate_syntax_tree(&args[1]);

    // Cairo language spec
    // cairo_lang_syntax_codegen::cairo_spec::get_spec
}

fn generate_syntax_tree(cairo_filename: &str) {
    let db_val = SimpleParserDatabase::default();
    let db = &db_val;

    let (syntax_root, _diagnostics) = get_syntax_root_and_diagnostics_from_file(db, cairo_filename);

    let printed_tree = run_printer(db, &syntax_root, MarkdownEngine::new());
    println!("{printed_tree}");
}
