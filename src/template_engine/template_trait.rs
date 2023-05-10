use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::SyntaxNode;

pub trait TemplateEngine {
    fn token(&mut self, description: &str, kind: &SyntaxKind, text: &str, node: &SyntaxNode);
    fn node_start(&mut self, description: &str, kind: &SyntaxKind, node: &SyntaxNode);
    fn node_end(&mut self, description: &str, kind: &SyntaxKind, node: &SyntaxNode);
    fn get_result(&self) -> String;
}
