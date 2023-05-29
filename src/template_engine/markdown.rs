use std::collections::HashMap;

use crate::SyntaxGroup;
use crate::SyntaxKind;
use crate::SyntaxKind::*;
use crate::SyntaxNode;

use super::TemplateEngine;

pub struct MarkdownEngine {
    pub templates: HashMap<String, String>,
    nodes: Vec<(SyntaxKind, String, usize)>,
    tokens: Vec<(SyntaxKind, String, String)>,
    pub ignored_nodes: HashMap<SyntaxKind, u8>,
    payload: String,
}

impl TemplateEngine for MarkdownEngine {
    fn init(&mut self, _db: &dyn SyntaxGroup) {}

    fn token(&mut self, description: &str, text: &str, node: &SyntaxNode, db: &dyn SyntaxGroup) {
        let kind = node.kind(db);
        if self.ignored_nodes.contains_key(&kind) {
            return;
        }
        let text = match kind {
            SyntaxKind::TokenNewline => ".",
            _ => text,
        };

        self.tokens.push((kind, description.into(), text.into()));
    }

    fn node_start(&mut self, description: &str, node: &SyntaxNode, db: &dyn SyntaxGroup) {
        let kind = node.kind(db);
        if self.ignored_nodes.contains_key(&kind) {
            return;
        }

        self.nodes.push((kind, description.to_string(), self.tokens.len()));
    }

    fn node_end(&mut self, _description: &str, node: &SyntaxNode, db: &dyn SyntaxGroup) {
        let kind = node.kind(db);
        if self.ignored_nodes.contains_key(&kind) {
            return;
        }
        let node_tup = self.nodes.pop().unwrap();
        match kind {
            FunctionWithBody => self.process_function_doc(node_tup, node, db),
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

    pub fn process_function_doc(
        &mut self,
        node_tup: (SyntaxKind, String, usize),
        node: &SyntaxNode,
        db: &dyn SyntaxGroup,
    ) {
        // Gets all children nodes
        let tokens = &self.tokens[node_tup.2..];
        let max_index = tokens.len();
        let mut i: usize = 0;

        let mut function_name = String::new();
        let mut function_comments = String::new();
        let mut function_args = String::new();
        let mut function_return = String::new();

        while i < max_index {
            let (kind, _desc, text) = &tokens[i];
            if TokenSingleLineComment != *kind {
                break;
            }
            function_comments.push_str(&text.to_string().replace("//", "").trim());
            function_comments.push_str("\n");
            i += 1;
        }

        if !function_comments.is_empty() {
            function_comments = format!("\n{function_comments}\n");
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

        if !function_args.is_empty() {
            function_args = format!("\n\n#### Parameters:\n{function_args}\n");
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
                    function_return.push_str(text);
                    function_return.push(' ');
                }
            }
            i += 1;
        }

        if !function_return.is_empty() {
            function_return = format!("\n\n#### Returns:\n{function_return}\n");
        }

        let mut code = "".to_string();
        node.children(db).for_each(|x| code.push_str(&x.get_text(db)));
        code = code.trim_matches('\n').to_string();

        self.payload.push_str(&format!("\n## Function `{function_name}`\n"));
        self.payload.push_str(&format!("{function_comments}{function_args}{function_return}"));
        self.payload.push_str(&format!("\n#### Source code\n```rust\n{code}\n```\n"));
        self.payload.push_str(&format!("\n-----------------------------\n"));
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
