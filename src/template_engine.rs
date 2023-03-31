use std::collections::HashMap;
use std::u8;

use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::kind::SyntaxKind::*;

pub struct TemplateEngine {
    pub templates: HashMap<String, String>,
    nodes: Vec<(SyntaxKind, String, usize)>,
    tokens: Vec<(SyntaxKind, String, String)>,
    pub ignored_kinds: HashMap<SyntaxKind, u8>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        let mut ignored_nodes: HashMap<SyntaxKind, u8> = HashMap::new();
        // ignored_nodes.contains_key("");
        ignored_nodes.insert(ItemList, 0);
        ignored_nodes.insert(TokenNewline, 0);
        ignored_nodes.insert(SyntaxFile, 0);
        ignored_nodes.insert(Trivia, 0);
        ignored_nodes.insert(TokenWhitespace, 0);
        ignored_nodes.insert(TokenNewline, 0);
        ignored_nodes.insert(TokenNewline, 0);
        Self {
            templates: HashMap::new(),
            nodes: Vec::new(),
            tokens: Vec::new(),
            ignored_kinds: ignored_nodes,
        }
    }
    pub fn parse_token(&mut self, field_description: &str, text: &str, kind: &SyntaxKind) {
        if self.ignored_kinds.contains_key(&kind) {
            return;
        }
        let text = match kind {
            SyntaxKind::TokenNewline => ".",
            _ => text,
        };
        self.tokens.push((*kind, field_description.into(), text.into()));
    }
    pub fn node_start(&mut self, field_description: &str, kind: &SyntaxKind) {
        if self.ignored_kinds.contains_key(&kind) {
            return;
        }
        let _node_data = (kind.to_string(), field_description.to_string());
        self.nodes.push((*kind, field_description.to_string(), self.tokens.len()));
    }
    pub fn node_end(&mut self, _field_description: &str, kind: &SyntaxKind) {
        if self.ignored_kinds.contains_key(&kind) {
            return;
        }
        let node = self.nodes.pop().unwrap();
        match kind {
            FunctionWithBody => self.process_function_doc(node),
            _ => {}
        }
    }

    pub fn process_function_doc(&mut self, node: (SyntaxKind, String, usize)) {
        let tokens = &self.tokens[node.2..];
        let max_index = tokens.len();
        let mut i: usize = 0;

        let mut function_name = String::new();
        let mut function_comments = String::new();
        let mut function_args = String::new();
        let mut function_return = String::from("Void");

        while i < max_index {
            let (kind, _desc, text) = &tokens[i];
            if TokenSingleLineComment != *kind {
                break;
            }
            function_comments.push_str(&text.to_string().replace("//", "").trim());
            function_comments.push_str("\n");
            i += 1;
        }

        while i < max_index {
            let (kind, _, _) = &tokens[i];
            if TokenFunction == *kind {
                i += 1;
                break;
            }

            i += 1;
        }

        function_name.push_str(&tokens[i].2);

        while i < max_index {
            let (kind, _, _) = &tokens[i];
            if TokenLParen == *kind {
                i += 1;
                break;
            }
            i += 1;
        }

        while i < max_index {
            let (kind, _desc, text) = &tokens[i];
            if TokenRParen == *kind {
                i += 1;
                break;
            }
            match kind {
                TokenComma => {
                    function_args.push_str("\n");
                }
                _ => {
                    function_args.push_str(text);
                    function_args.push_str(" ");
                }
            }
            i += 1;
        }

        while i < max_index {
            let (kind, _desc, text) = &tokens[i];
            if TokenLBrace == *kind {
                break;
            }
            match kind {
                TokenArrow => {
                    function_return = "".into();
                }
                _ => {
                    if function_return != "Void" {
                        function_return.push_str(text);
                        function_return.push(' ');
                    }
                }
            }
            i += 1;
        }

        println!("\n\n");
        println!("### Function `{function_name}`");
        println!("{function_comments}");
        println!("");
        println!("#### Parameters:");
        println!("```");
        println!("{function_args}");
        println!("```");
        println!("");
        println!("#### Returns:");
        println!("```");
        println!("{function_return}");
        println!("```");
    }

    pub fn render_syntax_doc(
        &self,
        kind: SyntaxKind,
        desc: &str,
        text: &str,
        _parent_kind: SyntaxKind,
    ) -> String {
        match kind {
            TokenSingleLineComment => {
                format!("{kind:#?} {desc} {text}\n")
            }
            _ => {
                format!("{kind:#?} {desc} {text}\n")
            }
        }
    }
}
