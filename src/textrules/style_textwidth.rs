use crate::config::ConfigOption;
use crate::linter::{TextRule, TextRuleResult};

#[derive(Default)]
pub struct StyleTextwidth;

impl TextRule for StyleTextwidth {
    fn check(
        &mut self,
        line: &str,
        option: &ConfigOption,
    ) -> TextRuleResult {
        let char_indices: Vec<_> = line.char_indices().collect();
        let n_chars = char_indices.len();
        if n_chars > option.textwidth {
            TextRuleResult::Fail {
                offset: char_indices[option.textwidth].0,
                len: line.len() - char_indices[option.textwidth].0,
            }
        } else {
            TextRuleResult::Pass
        }
    }

    fn name(&self) -> String {
        String::from("style_textwidth")
    }

    fn hint(&self, option: &ConfigOption) -> String {
        String::from(format!(
            "Lines must be no longer than {} characters.",
            &option.textwidth
        ))
    }

    fn reason(&self) -> String {
        String::from("Excessively long lines cause problems with diffs and review.")
    }
}
