use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct UnpackedArray;

impl SyntaxRule for UnpackedArray {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> SyntaxRuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return SyntaxRuleResult::Pass;
            }
        };

        match node {
            RefNode::UnpackedDimension(_) => SyntaxRuleResult::Fail,
            _ => SyntaxRuleResult::Pass,
        }
    }
    fn name(&self) -> String {
        String::from("unpacked_array")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Avoid using unpacked arrays in variable declarations.")
    }

    fn reason(&self) -> String {
        String::from("Unpacked arrays can lead to issues during synthesis.")
    }
}
