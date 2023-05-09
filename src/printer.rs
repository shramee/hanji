// Source cairo/crates/cairo-lang-parser/src/printer.rs
use cairo_lang_syntax as syntax;
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::kind::SyntaxKind;
use cairo_lang_syntax::node::SyntaxNode;
use cairo_lang_syntax_codegen::cairo_spec::get_spec;
use cairo_lang_syntax_codegen::spec::{Member, Node, NodeKind};
use itertools::zip_eq;

use crate::template_engine::TemplateEngine;

pub fn run_printer(
    db: &dyn SyntaxGroup,
    syntax_root: &SyntaxNode,
    template_engine: impl TemplateEngine,
) -> String {
    let mut printer = Printer::new(db, template_engine);
    printer.print_tree("root", syntax_root, "", true, true);
    printer.template_engine.get_result()
}

pub struct Printer<'a, T: TemplateEngine> {
    template_engine: T,
    db: &'a dyn SyntaxGroup,
    spec: Vec<Node>,
    /// The highest SyntaxKind that is interesting. All other kinds, if not under it, are ignored.
    top_level_kind: Option<String>,
    /// Syntax kinds to ignore when printing. In this context, "ignore" means printing the nodes
    /// themselves, but not their children.
    ignored_kinds: Vec<String>,
}
impl<'a, T: TemplateEngine> Printer<'a, T> {
    fn new(db: &'a dyn SyntaxGroup, template_engine: T) -> Self {
        Self {
            db,
            template_engine,
            spec: get_spec(),
            top_level_kind: None,
            ignored_kinds: Vec::new(),
        }
    }

    /// `under_top_level`: whether we are in a subtree of the top-level kind.
    fn print_tree(
        &mut self,
        field_description: &str,
        syntax_node: &SyntaxNode,
        indent: &str,
        is_last: bool,
        under_top_level: bool,
    ) {
        let green_node = syntax_node.green_node(self.db);
        match green_node.details {
            syntax::node::green::GreenNodeDetails::Token(text) => {
                if under_top_level {
                    self.template_engine.parse_token(
                        field_description,
                        text.as_str(),
                        &green_node.kind,
                    );
                }
            }
            syntax::node::green::GreenNodeDetails::Node { .. } => {
                self.template_engine.node_start(field_description, &green_node.kind);
                self.process_internal_node(
                    field_description,
                    indent,
                    is_last,
                    syntax_node,
                    green_node.kind,
                    under_top_level,
                );
                self.template_engine.node_end(field_description, &green_node.kind);
            }
        };
    }

    /// `under_top_level`: whether we are in a subtree of the top-level kind.
    #[allow(clippy::too_many_arguments)]
    fn process_internal_node(
        &mut self,
        field_description: &str,
        indent: &str,
        is_last: bool,
        syntax_node: &SyntaxNode,
        kind: SyntaxKind,
        under_top_level: bool,
    ) {
        let current_is_top_level =
            !under_top_level && self.top_level_kind == Some(format!("{kind:?}"));
        // Update under_top_level and indent as needed.
        let (under_top_level, indent) =
            if current_is_top_level { (true, "") } else { (under_top_level, indent) };

        if let Some(token_node) = syntax_node.get_terminal_token(self.db) {
            self.print_tree(field_description, &token_node, indent, is_last, under_top_level);
            return;
        }

        let children: Vec<_> = syntax_node.children(self.db).collect();
        let num_children = children.len();

        if under_top_level && self.ignored_kinds.contains(&format!("{kind:?}")) {
            return;
        }

        if num_children == 0 {
            return;
        }

        let extra_indent = if is_last || current_is_top_level { "    " } else { "│   " };
        let indent = String::from(indent) + extra_indent;
        let node_kind = self.get_node_kind(kind.to_string());
        match node_kind {
            NodeKind::Struct { members: expected_children }
            | NodeKind::Terminal { members: expected_children, .. } => {
                self.print_internal_struct(
                    &children,
                    &expected_children,
                    indent.as_str(),
                    under_top_level,
                );
            }
            NodeKind::List { .. } => {
                for (i, child) in children.iter().enumerate() {
                    self.print_tree(
                        format!("child #{i}").as_str(),
                        child,
                        indent.as_str(),
                        i == num_children - 1,
                        under_top_level,
                    );
                }
            }
            NodeKind::SeparatedList { .. } => {
                for (i, child) in children.iter().enumerate() {
                    let description = if i % 2 == 0 { "item" } else { "separator" };
                    self.print_tree(
                        format!("{description} #{}", i / 2).as_str(),
                        child,
                        indent.as_str(),
                        i == num_children - 1,
                        under_top_level,
                    );
                }
            }
            _ => panic!("This should never happen"),
        }
    }

    /// Assumes children and expected children are non-empty of the same length.
    /// `under_top_level`: whether we are in a subtree of the top-level kind.
    fn print_internal_struct(
        &mut self,
        children: &[SyntaxNode],
        expected_children: &[Member],
        indent: &str,
        under_top_level: bool,
    ) {
        let (last_child, non_last_children) = children.split_last().unwrap();
        let (last_expected_child, non_last_expected_children) =
            expected_children.split_last().unwrap();
        for (child, expected_child) in zip_eq(non_last_children, non_last_expected_children) {
            self.print_tree(&expected_child.name, child, indent, false, under_top_level);
        }
        self.print_tree(&last_expected_child.name, last_child, indent, true, under_top_level);
    }

    fn get_node_kind(&self, name: String) -> NodeKind {
        if let Some(node) = self.spec.iter().find(|x| x.name == name) {
            node.kind.clone()
        } else {
            panic!("Could not find spec for {name}")
        }
    }
}
