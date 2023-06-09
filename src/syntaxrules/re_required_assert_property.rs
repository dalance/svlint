use crate::config::ConfigOption;
use crate::linter::{check_regex, SyntaxRule, SyntaxRuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ReRequiredAssertProperty {
    re: Option<Regex>,
    under_statement: Option<SyntaxRuleResult>,
    under_concurrent_assertion_item_statement: Option<SyntaxRuleResult>,
}

impl SyntaxRule for ReRequiredAssertProperty {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> SyntaxRuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_required_assert_property).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::Statement(x) => {
                        self.under_statement =
                            if let (Some(_id), _, _) = &x.nodes {
                                Some(check_regex(true, unwrap_node!(*x, BlockIdentifier),
                                                 &syntax_tree, &self.re.as_ref().unwrap()))
                            } else {
                                None // No check on anonymous statements.
                            };
                    }
                    RefNode::ConcurrentAssertionItemStatement(x) => {
                        self.under_concurrent_assertion_item_statement =
                            if let (Some(_id), _) = &x.nodes {
                                Some(check_regex(true, unwrap_node!(*x, BlockIdentifier),
                                                 &syntax_tree, &self.re.as_ref().unwrap()))
                            } else {
                                None // No check on anonymous concurrent assertions.
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
                    RefNode::ConcurrentAssertionItemStatement(_) => {
                        self.under_concurrent_assertion_item_statement = None;
                    }
                    _ => ()
                }
                return SyntaxRuleResult::Pass;
            }
        };

        match node {
            RefNode::AssertPropertyStatement(_) => {
                match (self.under_statement, self.under_concurrent_assertion_item_statement) {
                    (Some(r), None) => r,
                    (None, Some(r)) => r,
                    _ => SyntaxRuleResult::Pass,
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("re_required_assert_property")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a concurrent assertion identifier matching regex `{}`.",
            &option.re_required_assert_property
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
