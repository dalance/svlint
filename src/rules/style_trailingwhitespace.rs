use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use regex::Regex;
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct StyleTrailingwhitespace {
    re: Option<Regex>,
}

impl Rule for StyleTrailingwhitespace {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(r"[ ]+[\n\v\f\r]").unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };
        match node {
            RefNode::WhiteSpace(x) => {
                let re = self.re.as_ref().unwrap();
                let t = syntax_tree.get_str(*x).unwrap();
                if re.is_match(&t) {
                    RuleResult::Fail
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("style_trailingwhitespace")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Remove trailing whitespace.")
    }

    fn reason(&self) -> String {
        String::from("Trailing whitespace leads to unnecessary awkwardness with version control.")
    }
}
