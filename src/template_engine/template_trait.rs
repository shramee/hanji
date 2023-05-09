use cairo_lang_syntax::node::kind::SyntaxKind;

pub trait TemplateEngine {
    fn parse_token(&mut self, field_description: &str, text: &str, kind: &SyntaxKind);
    fn node_start(&mut self, field_description: &str, kind: &SyntaxKind);
    fn node_end(&mut self, field_description: &str, kind: &SyntaxKind);
}
