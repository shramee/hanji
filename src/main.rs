pub mod printer;
use std::env;

use cairo_lang_parser::utils::{get_syntax_root_and_diagnostics_from_file, SimpleParserDatabase};
use printer::print_tree;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("File path to parse is required.")
    }

    println!("{:#?}", generate_syntax_tree(&args[1]));

    // Cairo language spec
    // cairo_lang_syntax_codegen::cairo_spec::get_spec
}

fn generate_syntax_tree(cairo_filename: &str) {
    let db_val = SimpleParserDatabase::default();
    let db = &db_val;

    let (syntax_root, _diagnostics) = get_syntax_root_and_diagnostics_from_file(db, cairo_filename);
    // let diagnostics_str = diagnostics.format(db);

    let printed_tree = print_tree(db, &syntax_root, false, true);
    println!("{printed_tree}");
}

// fn main() {
// let args: Vec<String> = env::args().collect();
//
// if args.len() < 2 {
// panic!("File path to parse is required.")
// }
// let tree = generate_syntax_tree(&args[1]);
// parse_syntax_file(tree);
// }
//
// fn generate_syntax_tree(cairo_filename: &str) -> SyntaxFile {
// let db_val = SimpleParserDatabase::default();
// let db = &db_val;
//
// let file_id = FileId::new(db, PathBuf::from(cairo_filename));
// let contents = fs::read_to_string(cairo_filename).unwrap();
// let (syntax_file, diagnostics) = get_syntax_file_and_diagnostics(db, file_id, &contents);
// syntax_file
// }
//
// fn parse_syntax_file(tree: SyntaxFile) {
// let db_val = SimpleParserDatabase::default();
// let db = &db_val;
//
// println!("{tree:#?}");
// }
//
// fn parse_syntax_node(node: SyntaxNode) {
// node.
// }
//
