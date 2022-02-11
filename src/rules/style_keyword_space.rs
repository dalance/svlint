use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use regex::Regex;
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct StyleKeywordSpace {
    re: Option<Regex>,
}

impl Rule for StyleKeywordSpace {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            let keywords =
                [ "package"
                , "interface"
                , "module"
                , "case"
                , "for"
                , "if"
                , "assign"
                , "always"
                , "always_ff"
                ].join("|");
            self.re = Some(Regex::new(format!("^({})($|  )", keywords).as_str()).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };

        match node {
            RefNode::Keyword(x) => {
                let re = self.re.as_ref().unwrap();
                let kw = syntax_tree.get_str(*x).unwrap();

                if re.is_match(&kw) {
                    RuleResult::Fail
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("style_keyword_space")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("keyword should be followed by a single space")
    }

    fn reason(&self) -> String {
        String::from("consistent style enhances readability")
    }
}
