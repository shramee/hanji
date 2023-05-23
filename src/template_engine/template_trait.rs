use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::SyntaxNode;

pub trait TemplateEngine {
    fn init(&mut self, db: &dyn SyntaxGroup);
    fn token(&mut self, description: &str, text: &str, node: &SyntaxNode, db: &dyn SyntaxGroup);
    fn node_start(&mut self, description: &str, node: &SyntaxNode, db: &dyn SyntaxGroup);
    fn node_end(&mut self, description: &str, node: &SyntaxNode, db: &dyn SyntaxGroup);
    fn get_result(&self) -> String;
}
