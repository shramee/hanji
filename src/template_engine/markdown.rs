use std::collections::HashMap;

use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::kind::SyntaxKind::*;
use cairo_lang_syntax::node::SyntaxNode;

use super::TemplateEngine;

pub struct MarkdownEngine {
    pub templates: HashMap<String, String>,
    nodes: Vec<(SyntaxKind, String, usize)>,
    tokens: Vec<(SyntaxKind, String, String)>,
    pub ignored_nodes: HashMap<SyntaxKind, u8>,
    payload: String,
}

impl TemplateEngine for MarkdownEngine {
    fn token(&mut self, description: &str, kind: &SyntaxKind, text: &str, _node: &SyntaxNode) {
        if self.ignored_nodes.contains_key(&kind) {
            return;
        }
        let text = match kind {
            SyntaxKind::TokenNewline => ".",
            _ => text,
        };
        self.tokens.push((*kind, description.into(), text.into()));
    }

    fn node_start(&mut self, description: &str, kind: &SyntaxKind, _node: &SyntaxNode) {
        if self.ignored_nodes.contains_key(&kind) {
            return;
        }
        let _node_data = (kind.to_string(), description.to_string());
        self.nodes.push((*kind, description.to_string(), self.tokens.len()));
    }

    fn node_end(&mut self, _description: &str, kind: &SyntaxKind, _node: &SyntaxNode) {
        if self.ignored_nodes.contains_key(&kind) {
            return;
        }
        let node = self.nodes.pop().unwrap();
        match kind {
            FunctionWithBody => self.process_function_doc(node),
            _ => {}
        }
    }

    fn get_result(&self) -> String {
        self.payload.to_string()
    }
}

impl MarkdownEngine {
    pub fn new() -> Self {
        let mut ignored_nodes: HashMap<SyntaxKind, u8> = HashMap::new();
        // ignored_nodes.contains_key("");
        // ignored_nodes.insert(ItemList, 0);
        ignored_nodes.insert(TokenNewline, 0);
        ignored_nodes.insert(SyntaxFile, 0);
        // ignored_nodes.insert(Trivia, 0);
        ignored_nodes.insert(TokenWhitespace, 0);
        ignored_nodes.insert(TokenNewline, 0);
        ignored_nodes.insert(TokenNewline, 0);
        Self {
            templates: HashMap::new(),
            nodes: Vec::new(),
            tokens: Vec::new(),
            ignored_nodes,
            payload: "".into(),
        }
    }

    pub fn process_function_doc(&mut self, node: (SyntaxKind, String, usize)) {
        // Gets all children nodes
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

        self.payload = "".to_string()
            + &self.payload
            + &format!("## Function `{function_name}`\n")
            + &format!("{function_comments}\n")
            + &format!("\n")
            + &format!("#### Parameters:\n")
            + &format!("```\n")
            + &format!("{function_args}\n")
            + &format!("```\n")
            + &format!("\n")
            + &format!("#### Returns:\n")
            + &format!("```\n")
            + &format!("{function_return}\n")
            + &format!("```\n");
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
