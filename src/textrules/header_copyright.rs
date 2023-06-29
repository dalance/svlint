use crate::config::ConfigOption;
use crate::linter::{TextRule, TextRuleResult};
use regex::Regex;

#[derive(Default)]
pub struct HeaderCopyright {
    re: Option<Regex>,
    linenum: usize,
}

impl TextRule for HeaderCopyright {
    fn check(
        &mut self,
        line: Option<&str>,
        option: &ConfigOption,
    ) -> TextRuleResult {
        let line: &str = if line.is_none() {
            self.linenum = 0;
            return TextRuleResult::Pass;
        } else {
            line.unwrap()
        };
        self.linenum += 1;

        if self.re.is_none() {
            let year = &option.copyright_year;
            let holder = &option.copyright_holder;
            let r = format!(r"(copyright|Copyright|COPYRIGHT)\s+\((c|C)\)\s+{year}\s+{holder}");
            self.re = Some(Regex::new(&r).unwrap());
        }
        let re = self.re.as_ref().unwrap();

        if self.linenum == option.copyright_linenum {
            let is_match: bool = re.is_match(line);
            if is_match {
                TextRuleResult::Pass
            } else {
                TextRuleResult::Fail {
                    offset: 0,
                    len: line.len(),
                }
            }
        } else {
            TextRuleResult::Pass
        }
    }

    fn name(&self) -> String {
        String::from("header_copyright")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Copyright notice must be present on line {}.",
            &option.copyright_linenum
        ))
    }

    fn reason(&self) -> String {
        String::from("Copyright notices are required for legal purposes.")
    }
}
