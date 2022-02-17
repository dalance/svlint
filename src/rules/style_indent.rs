use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use regex::Regex;
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct StyleIndent {
    re: Option<Regex>,
}

impl Rule for StyleIndent {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(r"(?P<nl>[\n\v\f\r]+)(?P<sp>[ ]*)").unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };
        match node {
            RefNode::WhiteSpace(x) => {
                let re = self.re.as_ref().unwrap();
                let t = syntax_tree.get_str(*x).unwrap();
                for cap in re.captures_iter(&t) {
                    let sp = &cap["sp"];
                    if 0 != (sp.len() % option.indent) {
                        return RuleResult::Fail;
                    }
                }
                return RuleResult::Pass;
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("style_indent")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "newline should be followed by a multiple of {} spaces",
            &option.indent
        ))
    }

    fn reason(&self) -> String {
        String::from("Naming convention simplifies audit.")
    }
}
