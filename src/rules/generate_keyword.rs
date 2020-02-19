use crate::linter::{Rule, RuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

pub struct GenerateKeyword;

impl Rule for GenerateKeyword {
    fn check(&mut self, _syntax_tree: &SyntaxTree, event: &NodeEvent) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Skip;
            }
        };
        match node {
            RefNode::GenerateRegion(_) => RuleResult::Fail,
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("generate_keyword")
    }

    fn hint(&self) -> String {
        String::from("`generate`/`endgenerate` must be omitted")
    }

    fn reason(&self) -> String {
        String::from("")
    }
}
