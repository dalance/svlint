use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

pub struct StyleOperatorIntegerLeadingSpace {
    /// Number of trailing spaces of the previous node
    prev_trailing_space: usize,

    /// True if the previous node is an `Expression`
    prev_is_expr: bool,
}

impl SyntaxRule for StyleOperatorIntegerLeadingSpace {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> SyntaxRuleResult {
        match event {
            NodeEvent::Enter(node) => {
                if !self.prev_is_expr {
                    return SyntaxRuleResult::Pass;
                }

                match node {
                    RefNode::BinaryOperator(x) => {
                        let t = syntax_tree.get_str_trim(*x).unwrap();

                        match t {
                            "&" | "|" | "^" | "^~" | "~^" | ">>" | "<<" | ">>>" | "<<<" => {
                                if self.prev_trailing_space == 1 {
                                    SyntaxRuleResult::Pass
                                } else {
                                    SyntaxRuleResult::Fail
                                }
                            }
                            _ => SyntaxRuleResult::Pass,
                        }
                    }
                    _ => SyntaxRuleResult::Pass,
                }
            }
            NodeEvent::Leave(node) => {
                match node {
                    RefNode::Expression(expr) => {
                        self.prev_is_expr = true;
                        let trailing_space =
                            count_trailing_space(syntax_tree.get_str(*expr).unwrap());
                        self.prev_trailing_space = trailing_space;
                    }
                    RefNode::ConstantExpression(expr) => {
                        self.prev_is_expr = true;
                        let trailing_space =
                            count_trailing_space(syntax_tree.get_str(*expr).unwrap());
                        self.prev_trailing_space = trailing_space;
                    }
                    _ => {
                        self.prev_is_expr = false;
                        self.prev_trailing_space = 0;
                    }
                }
                SyntaxRuleResult::Pass
            }
        }
    }

    fn name(&self) -> String {
        String::from("style_operator_integer_leading_space")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from(format!(
            "Put exact one space before binary integer operators."
        ))
    }

    fn reason(&self) -> String {
        String::from("Consistent use of whitespace enhances readability by reducing visual noise.")
    }
}

fn count_trailing_space(s: &str) -> usize {
    let mut count = 0;
    for c in s.chars().rev() {
        if c == ' ' {
            count += 1;
        } else {
            break;
        }
    }
    count
}

impl std::default::Default for StyleOperatorIntegerLeadingSpace {
    fn default() -> Self {
        Self {
            prev_trailing_space: 0,
            prev_is_expr: false,
        }
    }
}
