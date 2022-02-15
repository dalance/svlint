use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use regex::Regex;
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct StyleKeyword1Or2Space {
    re: Option<Regex>,
}

impl Rule for StyleKeyword1Or2Space {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            let keywords =
                [ "inout"
                , "input"
                ].join("|");

            /* Regex defines pattens which should fail.
            These keywords are in the same category as style_keyword_1space,
            but an exception is made for input/inout port directions because
            they're frequently aligned nicely with output port directions.

            Keyword followed by something other than exactly 1 or 2 spaces:
                - nothing
                - 3 (or more) spaces
                - something not a space
            */
            self.re = Some(Regex::new(format!("^({})($|   |[^ ])", keywords).as_str()).unwrap());
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
        String::from("style_keyword_1or2space")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("keyword should be followed by exactly 1 or 2 spaces")
    }

    fn reason(&self) -> String {
        String::from("consistent style enhances readability")
    }
}
