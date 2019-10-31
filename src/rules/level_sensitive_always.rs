use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, AlwaysKeyword, RefNode, SyntaxTree};

pub struct LevelSensitiveAlways;

impl Rule for LevelSensitiveAlways {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::AlwaysConstruct(x) => {
                let (ref a, ref b) = x.nodes;
                let always = unwrap_node!(a, AlwaysKeyword).unwrap();
                let edge = unwrap_node!(b, EdgeIdentifier);
                match always {
                    RefNode::AlwaysKeyword(AlwaysKeyword::Always(_)) => {
                        if edge.is_some() {
                            RuleResult::Pass
                        } else {
                            RuleResult::Fail
                        }
                    }
                    _ => RuleResult::Pass,
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("level sensitive always")
    }

    fn hint(&self) -> String {
        String::from("level sensitive 'always' must be 'always_comb'")
    }
}
