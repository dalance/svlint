use crate::config::ConfigOption;
use crate::linter::{check_regex, SyntaxRule, RuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ReRequiredPackage {
    re: Option<Regex>,
}

impl SyntaxRule for ReRequiredPackage {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_required_package).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };

        match node {
            RefNode::PackageDeclaration(x) => {
                check_regex(true, unwrap_node!(*x, PackageIdentifier),
                            &syntax_tree, &self.re.as_ref().unwrap())
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("re_required_package")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a package identifier matching regex `{}`.",
            &option.re_required_package
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
