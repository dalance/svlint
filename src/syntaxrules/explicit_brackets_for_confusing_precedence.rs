use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{Expression, ExpressionBinary, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ExplicitBracketsForConfusingPrecedence;

/// Recursively determine if a binary expression contains comparison (== etc.)
/// and bitwise (& etc.) operators. This does NOT recurse into bracketed
/// expressions.
fn find_binary_operators(
    syntax_tree: &SyntaxTree,
    exp_bin: &ExpressionBinary,
    has_comparison_op: &mut bool,
    has_bitwise_op: &mut bool,
) {
    // Stop recursing if we've found both already.
    if *has_comparison_op && *has_bitwise_op {
        return;
    }

    let bin_op = &exp_bin.nodes.1;
    let children = [&exp_bin.nodes.0, &exp_bin.nodes.3];

    let bin_op_loc = &bin_op.nodes.0.nodes.0;
    let bin_op_str = syntax_tree.get_str(bin_op_loc).unwrap();

    match bin_op_str {
        ">=" | ">" | "<" | "<=" | "==" | "!=" | "===" | "!==" | "==?" | "!=?" => {
            *has_comparison_op = true
        }
        "&" | "|" | "^" | "^~" | "~^" => *has_bitwise_op = true,
        _ => {}
    }

    for child in children {
        match child {
            Expression::Binary(bin) => {
                find_binary_operators(syntax_tree, bin, has_comparison_op, has_bitwise_op);
            }
            _ => {}
        }
    }
}

impl SyntaxRule for ExplicitBracketsForConfusingPrecedence {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> SyntaxRuleResult {
        match event {
            NodeEvent::Enter(RefNode::ExpressionBinary(exp_bin)) => {
                let mut has_comparison_op = false;
                let mut has_bitwise_op = false;

                find_binary_operators(
                    syntax_tree,
                    exp_bin,
                    &mut has_comparison_op,
                    &mut has_bitwise_op,
                );

                if has_comparison_op && has_bitwise_op {
                    SyntaxRuleResult::Fail
                } else {
                    SyntaxRuleResult::Pass
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("explicit_brackets_for_confusing_precedence")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Add brackets to avoid ungrouped mix of comparison and bitwise operators.")
    }

    fn reason(&self) -> String {
        String::from("Avoids mistakes from assuming intuitive operator precedence.")
    }
}
