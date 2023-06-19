use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use regex::Regex;
use sv_parser::{NodeEvent, RefNode, SyntaxTree, WhiteSpace};

#[derive(Default)]
pub struct StyleTrailingwhitespace {
    re: Option<Regex>,
    buffer: String,
}

impl SyntaxRule for StyleTrailingwhitespace {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> SyntaxRuleResult {
        if self.re.is_none() {
            self.re = Some(Regex::new(r"[ \t]+[\n\v\f\r]").unwrap());
        }

        let node = match event {
            NodeEvent::Enter(n) => {
                match n {
                    RefNode::WhiteSpace(WhiteSpace::Comment(_)) |
                    RefNode::WhiteSpace(WhiteSpace::Newline(_)) |
                    RefNode::WhiteSpace(WhiteSpace::Space(_)) => {
                        // Keep only the final character from the previous element.
                        let last_char: Option<char> = self.buffer.pop();
                        self.buffer.clear();
                        if let Some(c) = last_char {
                            self.buffer.push(c);
                        }
                    }
                    RefNode::Comment(_) |
                    RefNode::Locate(_) => {
                        // No change to buffer within Vec<WhiteSpace> elements.
                    }
                    _ => {
                        // Clear the buffer on exit from Vec<WhiteSpace>.
                        self.buffer.clear();
                    }
                }

                // Append this node's string to buffer.
                match n {
                    RefNode::WhiteSpace(WhiteSpace::Space(x)) => {
                        self.buffer.push_str(syntax_tree.get_str(x).unwrap());
                    }
                    RefNode::WhiteSpace(WhiteSpace::Newline(x)) => {
                        self.buffer.push_str(syntax_tree.get_str(x).unwrap());
                    }
                    RefNode::WhiteSpace(WhiteSpace::Comment(x)) => {
                        self.buffer.push_str(syntax_tree.get_str(x).unwrap());
                    }
                    _ => {}
                }

                n
            }
            NodeEvent::Leave(_) => {
                return SyntaxRuleResult::Pass;
            }
        };
        match node {
            RefNode::WhiteSpace(_) => {
                let re = self.re.as_ref().unwrap();
                if re.is_match(&self.buffer) {
                    SyntaxRuleResult::Fail
                } else {
                    SyntaxRuleResult::Pass
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("style_trailingwhitespace")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Remove trailing whitespace.")
    }

    fn reason(&self) -> String {
        String::from("Trailing whitespace leads to unnecessary awkwardness with version control.")
    }
}
