use crate::config::ConfigOption;
use crate::linter::{check_regex, Rule, RuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ReForbiddenAssert {
    re: Option<Regex>,
    under_statement: Option<RuleResult>,
}

impl Rule for ReForbiddenAssert {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_forbidden_assert).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::Statement(x) => {
                        self.under_statement =
                            if let (Some(_id), _, _) = &x.nodes {
                                Some(check_regex(false, unwrap_node!(*x, BlockIdentifier),
                                                 &syntax_tree, &self.re.as_ref().unwrap()))
                            } else {
                                None // No check on anonymous statements.
                            };
                    }
                    _ => ()
                }
                x
            }
            NodeEvent::Leave(x) => {
                match x {
                    RefNode::Statement(_) => {
                        self.under_statement = None;
                    }
                    _ => ()
                }
                return RuleResult::Pass;
            }
        };

        match node {
            RefNode::DeferredImmediateAssetionItem(x) => {
                if let (Some(_id), _) = &x.nodes {
                    check_regex(false, unwrap_node!(*x, BlockIdentifier),
                                &syntax_tree, &self.re.as_ref().unwrap())
                } else {
                    RuleResult::Pass // No check on anonymous immediate assertions.
                }
            }
            RefNode::SimpleImmediateAssertStatement(_) |
            RefNode::DeferredImmediateAssertStatement(_) => {
                match self.under_statement {
                    Some(r) => r,
                    None => RuleResult::Pass,
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("re_forbidden_assert")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use an immediate assertion identifier not matching regex \"{}\".",
            &option.re_forbidden_assert
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
