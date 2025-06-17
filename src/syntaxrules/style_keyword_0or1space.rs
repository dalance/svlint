use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use regex::Regex;
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct StyleKeyword0Or1Space {
    re_split: Option<Regex>,
    re_kw: Option<Regex>,
    re_succ: Option<Regex>,
}

impl SyntaxRule for StyleKeyword0Or1Space {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> SyntaxRuleResult {
        /*
        re_split extracts keyword from anything following it.
        re_kw is used to selectively apply this rule to specific keywords.
        re_succ matches what is allowed after the keyword.
            - nothing, immediately followed by symbol
            - exactly 1space, then identifier or symbol
        */
        if self.re_split.is_none() {
            self.re_split = Some(Regex::new(r#"(?P<kw>[\\$'"BXZa-z_01]+)(?P<succ>(?s:.)*)"#).unwrap());
        }
        if self.re_kw.is_none() {
            let keywords =
                [ "return" // {{{
                ].join("|"); // }}}

            self.re_kw = Some(Regex::new(format!("^({})$", keywords).as_str()).unwrap());
        }
        if self.re_succ.is_none() {
            self.re_succ = Some(Regex::new(r"^[ ]?$").unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return SyntaxRuleResult::Pass;
            }
        };

        match node {
            RefNode::Keyword(x) => {
                let re_split = self.re_split.as_ref().unwrap();
                let re_kw = self.re_kw.as_ref().unwrap();
                let t = syntax_tree.get_str(*x).unwrap();
                if let Some(caps) = re_split.captures(&t) {
                    if re_kw.is_match(&caps[1]) {
                        let re_succ = self.re_succ.as_ref().unwrap();

                        if re_succ.is_match(&caps[2]) {
                            SyntaxRuleResult::Pass
                        } else {
                            SyntaxRuleResult::Fail
                        }
                    } else {
                        SyntaxRuleResult::Pass
                    }
                } else {
                    // Don't crash in case there's a mistake in the `re_split` regex
                    // and it doesn't match a `Keyword`.
                    SyntaxRuleResult::Pass
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("style_keyword_0or1space")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Follow keyword with a symbol or exactly 1 space.")
    }

    fn reason(&self) -> String {
        String::from("Consistent use of whitespace enhances readability by reducing visual noise.")
    }
}
