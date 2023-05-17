use crate::config::ConfigOption;
use crate::linter::{check_regex, SyntaxRule, RuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ReRequiredGenerateblock {
    re: Option<Regex>,
    under_generate_block: bool,
}

impl SyntaxRule for ReRequiredGenerateblock {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_required_generateblock).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::GenerateBlock(_) => {
                        self.under_generate_block = true;
                    }
                    _ => ()
                }
                x
            }
            NodeEvent::Leave(x) => {
                match x {
                    RefNode::GenerateBlock(_) => {
                        self.under_generate_block = false;
                    }
                    _ => ()
                }
                return RuleResult::Pass;
            }
        };

        match (self.under_generate_block, node) {
            (true, RefNode::GenerateBlockIdentifier(x)) => {
                check_regex(true, unwrap_node!(*x, Identifier),
                            &syntax_tree, &self.re.as_ref().unwrap())
            }
            _ => RuleResult::Pass
        }
    }

    fn name(&self) -> String {
        String::from("re_required_generateblock")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a generate block identifier matching regex `{}`.",
            &option.re_required_generateblock
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
