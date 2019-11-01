use crate::linter::{Rule, RuleResult};
use sv_parser::{RefNode, SyntaxTree};

pub struct EnumWithType;

impl Rule for EnumWithType {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::DataTypeEnum(x) => {
                let (_, ref a, _, _) = x.nodes;
                if a.is_some() {
                    RuleResult::Pass
                } else {
                    RuleResult::Fail
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("enum with type")
    }

    fn hint(&self) -> String {
        String::from("`enum` must have data type")
    }

    fn reason(&self) -> String {
        String::from("the default data type is `int`")
    }
}
