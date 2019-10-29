use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, unwrap_node, RefNode, SyntaxTree};

pub struct ParameterInPackage;

impl Rule for ParameterInPackage {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::PackageDeclaration(x) => {
                let param = unwrap_node!(x.clone(), ParameterDeclaration);
                if let Some(param) = param {
                    let param_locate = unwrap_locate!(param).unwrap();
                    RuleResult::FailLocate(param_locate.clone())
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("parameter in package")
    }

    fn hint(&self) -> String {
        String::from("'parameter' must be replaced to 'localparam' in package")
    }
}
