use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
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
    ) -> SyntaxRuleResult {
        match event {
            NodeEvent::Enter(RefNode::GenerateRegion(_)) => {
                self.generate_region.push(());
                SyntaxRuleResult::Pass
            }
            NodeEvent::Leave(RefNode::GenerateRegion(_)) => {
                self.generate_region.pop();
                SyntaxRuleResult::Pass
            }
            NodeEvent::Enter(RefNode::IfGenerateConstruct(_)) => {
                if self.generate_region.last().is_some() {
                    SyntaxRuleResult::Pass
                } else {
                    SyntaxRuleResult::Fail
                }
            }
            NodeEvent::Enter(RefNode::CaseGenerateConstruct(_)) => {
                if self.generate_region.last().is_some() {
                    SyntaxRuleResult::Pass
                } else {
                    SyntaxRuleResult::Fail
                }
            }
            NodeEvent::Enter(RefNode::LoopGenerateConstruct(_)) => {
                if self.generate_region.last().is_some() {
                    SyntaxRuleResult::Pass
                } else {
                    SyntaxRuleResult::Fail
                }
            }
            _ => SyntaxRuleResult::Pass,
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
