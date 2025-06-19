use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use regex::Regex;
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct StyleKeyword1SpaceOrNewline {
    re_split: Option<Regex>,
    re_kw: Option<Regex>,
    re_succ: Option<Regex>,
}

impl SyntaxRule for StyleKeyword1SpaceOrNewline {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _config: &ConfigOption,
    ) -> SyntaxRuleResult {
        let re_split = &*self.re_split.get_or_insert_with(|| {
            Regex::new(r"(?P<kw>[\\$'BXZa-z_01]+)(?P<succ>(?s:.)*)").unwrap()
        });
        let re_kw = &*self.re_kw.get_or_insert_with(|| {
            let keywords = [
                "matches", // {{{
            ]
            .join("|"); // }}}
            Regex::new(format!("^({})$", keywords).as_str()).unwrap()
        });
        let re_succ = &*self
            .re_succ
            .get_or_insert_with(|| Regex::new(r"^([\n\v\f\r]| /| [^ ]?$)").unwrap());

        match event {
            NodeEvent::Enter(RefNode::Keyword(x)) => {
                let t = syntax_tree.get_str(*x).unwrap();
                let caps = re_split.captures(t).unwrap();

                if re_kw.is_match(&caps[1]) {
                    if re_succ.is_match(&caps[2]) {
                        SyntaxRuleResult::Pass
                    } else {
                        SyntaxRuleResult::Fail
                    }
                } else {
                    SyntaxRuleResult::Pass
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("style_keyword_1spaceornewline")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Follow keyword with a newline or exactly 1 space.")
    }

    fn reason(&self) -> String {
        String::from("Consistent use of whitespace enhances readability by reducing visual noise.")
    }
}
