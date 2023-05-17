use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, RuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct KeywordRequiredGenerate {
    generate_region: Vec<()>,
}

impl SyntaxRule for KeywordRequiredGenerate {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        match event {
            NodeEvent::Enter(RefNode::GenerateRegion(_)) => {
                self.generate_region.push(());
                RuleResult::Pass
            }
            NodeEvent::Leave(RefNode::GenerateRegion(_)) => {
                self.generate_region.pop();
                RuleResult::Pass
            }
            NodeEvent::Enter(RefNode::IfGenerateConstruct(_)) => {
                if self.generate_region.last().is_some() {
                    RuleResult::Pass
                } else {
                    RuleResult::Fail
                }
            }
            NodeEvent::Enter(RefNode::CaseGenerateConstruct(_)) => {
                if self.generate_region.last().is_some() {
                    RuleResult::Pass
                } else {
                    RuleResult::Fail
                }
            }
            NodeEvent::Enter(RefNode::LoopGenerateConstruct(_)) => {
                if self.generate_region.last().is_some() {
                    RuleResult::Pass
                } else {
                    RuleResult::Fail
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("keyword_required_generate")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Use `generate`/`endgenerate` keywords to define generate regions.")
    }

    fn reason(&self) -> String {
        String::from("Omitting `generate`/`endgenerate` keywords may cause issues with non-compliant tools.")
    }
}
