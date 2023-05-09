use std::env;

use cairo_lang_parser::printer::print_tree;
use cairo_lang_parser::utils::{get_syntax_root_and_diagnostics_from_file, SimpleParserDatabase};
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("File path to parse is required.")
    }
    generate_syntax_tree(&args[1]);
}

fn generate_syntax_tree(cairo_filename: &str) {
    let db_val = SimpleParserDatabase::default();
    let db = &db_val;

    let (syntax_root, _diagnostics) = get_syntax_root_and_diagnostics_from_file(db, cairo_filename);

    let _printed_tree = print_tree(db, &syntax_root, false, true);

    println!("{_printed_tree}");
}
