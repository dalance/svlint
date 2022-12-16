use crate::config::ConfigOption;
use crate::linter::{check_regex, Rule, RuleResult};
use regex::Regex;
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ReRequiredTask {
    re: Option<Regex>,
}

impl Rule for ReRequiredTask {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(&option.re_required_task).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };

        match node {
            RefNode::TaskDeclaration(x) => {
                check_regex(true, unwrap_node!(*x, TaskIdentifier),
                            &syntax_tree, &self.re.as_ref().unwrap())
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("re_required_task")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Use a task identifier matching regex \"{}\".",
            &option.re_required_task
        ))
    }

    fn reason(&self) -> String {
        String::from("Identifiers must conform to the naming scheme.")
    }
}
