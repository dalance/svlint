use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use regex::Regex;
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct StyleIndent {
    re: Option<Regex>,
}

impl SyntaxRule for StyleIndent {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> SyntaxRuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(r"(?P<nl>[\n\v\f\r]+)(?P<sp>[ ]*)").unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return SyntaxRuleResult::Pass;
            }
        };
        match node {
            RefNode::WhiteSpace(x) => {
                let re = self.re.as_ref().unwrap();
                let t = syntax_tree.get_str(*x).unwrap();
                for cap in re.captures_iter(&t) {
                    let sp = &cap["sp"];
                    if 0 != (sp.len() % option.indent) {
                        return SyntaxRuleResult::Fail;
                    }
                }
                return SyntaxRuleResult::Pass;
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("style_indent")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Follow each newline with an integer multiple of {} spaces.",
            &option.indent
        ))
    }

    fn reason(&self) -> String {
        String::from("Consistent indentation is essential for readability.")
    }
}
