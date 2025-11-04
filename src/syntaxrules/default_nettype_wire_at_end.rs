use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree, Locate, unwrap_locate};

#[derive(Default)]
pub struct DefaultNettypeWireAtEnd {
    last_nettype_value: Option<String>,
    last_nettype_loc: Option<Locate>,
}

impl SyntaxRule for DefaultNettypeWireAtEnd {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> SyntaxRuleResult {
        match event {
            NodeEvent::Enter(x) => {
                // Reset state when entering a new source file
                if let RefNode::SourceText(_) = x {
                    self.last_nettype_value = None;
                    self.last_nettype_loc = None;
                }

                // Capture the value and location of each `default_nettype` directive
                if let RefNode::DefaultNettypeCompilerDirective(directive) = x {
                    let (_symbol, _keyword, default_nettype_value) = &directive.nodes;
                    if let Some(s) = syntax_tree.get_str_trim(default_nettype_value) {
                        self.last_nettype_value = Some(s.to_string());
                        self.last_nettype_loc = unwrap_locate!(default_nettype_value).cloned();
                    } else {
                        // Fail if directive exists but its value cannot be parsed
                        if let Some(loc) = unwrap_locate!(default_nettype_value) {
                            return SyntaxRuleResult::FailLocate(loc.clone());
                        }
                        return SyntaxRuleResult::Fail;
                    }
                }
                SyntaxRuleResult::Pass
            }
            NodeEvent::Leave(x) => {
                // At the end of the source file, check the final effective nettype
                if let RefNode::SourceText(_) = x {
                    match self.last_nettype_value.as_deref() {
                        // Pass if the final nettype is "wire"
                        Some("wire") => SyntaxRuleResult::Pass,
                        // Pass if no directive is present at all (rule not applicable)
                        None => SyntaxRuleResult::Pass,
                        // Fail for any other value (e.g., "none")
                        Some(_) => {
                            if let Some(loc) = &self.last_nettype_loc {
                                SyntaxRuleResult::FailLocate(loc.clone())
                            } else {
                                SyntaxRuleResult::Fail
                            }
                        }
                    }
                } else {
                    SyntaxRuleResult::Pass
                }
            }
        }
    }

    fn name(&self) -> String {
        String::from("default_nettype_wire_at_end")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Ensure the final effective `default_nettype` is exactly `wire`.")
    }

    fn reason(&self) -> String {
        String::from("Directive `default_nettype wire` restores default nettype to prevent cross-file leaks.")
    }
}

