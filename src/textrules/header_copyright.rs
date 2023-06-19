use crate::config::ConfigOption;
use crate::linter::{TextRule, TextRuleResult};
use regex::Regex;

#[derive(Default)]
pub struct HeaderCopyright {
    re: Option<Regex>,
    linenum: Option<usize>,
}

impl TextRule for HeaderCopyright {
    fn check(
        &mut self,
        line: &str,
        option: &ConfigOption,
    ) -> TextRuleResult {
        if self.re.is_none() {
            let year = &option.copyright_year;
            let holder = &option.copyright_holder;
            let r = format!(r"(copyright|Copyright|COPYRIGHT)\s+\((c|C)\)\s+{year}\s+{holder}");
            self.re = Some(Regex::new(&r).unwrap());
        }
        let re = self.re.as_ref().unwrap();

        if self.linenum.is_none() {
            self.linenum = Some(0);
        }
        if let Some(x) = self.linenum {
            self.linenum = Some(x+1)
        }

        if self.linenum.unwrap() == option.copyright_linenum {
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
