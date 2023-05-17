use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, RuleResult};
use regex::Regex;
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct StyleOperatorUnary {
    re_succ: Option<Regex>,
}

impl SyntaxRule for StyleOperatorUnary {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        /*
        re_succ matches the unary symbol, then what is allowed after the
        operator symbol.
            - nothing, immediately followed by a symbol or identifier.
        */
        if self.re_succ.is_none() {
            self.re_succ = Some(Regex::new(r"^\S+$").unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };

        let s: Option<&str> = match node {
            RefNode::UnaryOperator(x) => {
                Some(syntax_tree.get_str(*x).unwrap())
            }
            RefNode::UnaryModulePathOperator(x) => {
                Some(syntax_tree.get_str(*x).unwrap())
            }
            RefNode::IncOrDecOperator(x) => {
                Some(syntax_tree.get_str(*x).unwrap())
            }
            _ => None,
        };

        if let Some(t) = s {
            let re_succ = self.re_succ.as_ref().unwrap();
            if re_succ.is_match(t) {
                RuleResult::Pass
            } else {
                RuleResult::Fail
            }
        } else {
            RuleResult::Pass
        }
    }

    fn name(&self) -> String {
        String::from("style_operator_unary")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Remove all whitespace following the operator.")
    }

    fn reason(&self) -> String {
        String::from("Consistent use of whitespace enhances readability by reducing visual noise.")
    }
}
