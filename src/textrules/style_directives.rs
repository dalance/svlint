use crate::config::ConfigOption;
use crate::linter::{TextRule, TextRuleResult};
use regex::Regex;

#[derive(Default)]
pub struct StyleDirectives {
    re: Option<Regex>,
}

impl TextRule for StyleDirectives {
    fn check(
        &mut self,
        line: &str,
        _option: &ConfigOption,
    ) -> TextRuleResult {
        if self.re.is_none() {
            let keywords =
                [ "begin_keywords" // {{{
                , "end_keywords"
                , "celldefine"
                , "endcelldefine"
                , "unconnected_drive"
                , "nounconnected_drive"
                , "pragma"
                , "timescale"
                , "default_nettype"
                , "line"
                , "resetall"
                , "include"
                , "define"
                , "ifdef"
                , "ifndef"
                , "elsif"
                , "else"
                , "endif"
                , "undef"
                , "undefineall"
                ].join("|"); // }}}

            self.re = Some(Regex::new(format!("^(.+)`({})", keywords).as_str()).unwrap());
        }
        let re = self.re.as_ref().unwrap();

        if let Some(caps) = re.captures(line) {
            if let Some(m) = caps.get(1) {
                TextRuleResult::Fail {
                    offset: 0,
                    len: m.as_str().chars().count(),
                }
            } else {
                TextRuleResult::Pass
            }
        } else {
            TextRuleResult::Pass
        }
    }

    fn name(&self) -> String {
        String::from("style_directives")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from(format!("Remove whitespace preceeding compiler directive."))
    }

    fn reason(&self) -> String {
        String::from("Compiler directives should not cause whitespace issues or hide in other code.")
    }
}
