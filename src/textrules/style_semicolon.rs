use crate::config::ConfigOption;
use crate::linter::{TextRule, TextRuleEvent, TextRuleResult};
use regex::Regex;

#[derive(Default)]
pub struct StyleSemicolon {
    re: Option<Regex>,
}

impl TextRule for StyleSemicolon {
    fn check(
        &mut self,
        event: TextRuleEvent,
        _option: &ConfigOption,
    ) -> TextRuleResult {
        let line: &str = match event {
            TextRuleEvent::StartOfFile => {
                return TextRuleResult::Pass;
            }
            TextRuleEvent::Line(x) => x,
        };

        if self.re.is_none() {
            self.re = Some(Regex::new("([ ]+);").unwrap());
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
        String::from("style_semicolon")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from(format!("Remove whitespace preceeding semicolon."))
    }

    fn reason(&self) -> String {
        String::from("Whitespace before a semicolon may obfuscate the statement.")
    }
}
