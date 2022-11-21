use crate::config::ConfigOption;
use crate::linter::{check_regex, Rule, RuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree, StatementItem, ProceduralAssertionStatement};

#[derive(Default)]
pub struct ReRequiredAssert {
    re: Option<Regex>,
}

impl Rule for ReRequiredAssert {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_required_assert).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };

        match node {
            RefNode::DeferredImmediateAssetionItem(x) => {
                if let (Some(_id), _) = &x.nodes {
                    check_regex(true, unwrap_node!(*x, BlockIdentifier),
                                &syntax_tree, &self.re.as_ref().unwrap())
                } else {
                    RuleResult::Pass // No check on anonymous immediate assertions.
                }
            }
            RefNode::StatementItem(x) => {
                println!("STATEMENT ITEM");
                match x {
                    StatementItem::ProceduralAssertionStatement(ProceduralAssertionStatement::Immediate(y)) => {
                        println!("IMMEDIATE");
                        RuleResult::Fail
                    }
                    _ => RuleResult::Pass,
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("re_required_assert")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use an immediate assertion identifier matching regex \"{}\".",
            &option.re_required_assert
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
