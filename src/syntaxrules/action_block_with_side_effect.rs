use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use regex::Regex;
use sv_parser::{unwrap_locate, unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct ActionBlockWithSideEffect {
    re: Option<Regex>,
}

impl SyntaxRule for ActionBlockWithSideEffect {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> SyntaxRuleResult {
        if self.re.is_none() {
            let io_tasks =
                [ "[f]?display[bho]?" // {{{
                , "[f]?strobe[bho]?"
                , "[sf]?write[bho]?"
                , "[f]?monitor[bho]?"
                , "monitoroff"
                , "monitoron"
                , "fclose"
                , "fopen"
                , "[sf]scanf"
                , "fread"
                , "fseek"
                , "fflush"
                , "feof"
                , "fget[sc]"
                , "ungetc"
                , "rewind"
                , "ftell"
                , "ferror"
                , "readmem[bh]"
                , "writemem[bh]"
                , "dumpfile"
                , "dumpvars"
                , "dumpoff"
                , "dumpon"
                , "dumpall"
                , "dumplimit"
                , "dumpflush"
                , "dumpports"
                , "dumpportsoff"
                , "dumpportson"
                , "dumpportsall"
                , "dumpportsflush"
                , "dumpportslimit"
                ].join("|"); // }}}

            self.re = Some(Regex::new(format!("^\\$({})$", io_tasks).as_str()).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return SyntaxRuleResult::Pass;
            }
        };

        match node {
            RefNode::ActionBlock(x) => {
                let loc = if let Some(y) = unwrap_node!(*x, BlockingAssignment) {
                    Some(unwrap_locate!(y).unwrap())
                } else if let Some(y) = unwrap_node!(*x, VariableDeclAssignment) {
                    Some(unwrap_locate!(y).unwrap())
                } else if let Some(y) = unwrap_node!(*x, IncOrDecExpression) {
                    Some(unwrap_locate!(y).unwrap())
                } else if let Some(y) = unwrap_node!(*x, SystemTfIdentifier) {
                    let re = self.re.as_ref().unwrap();
                    let l = unwrap_locate!(y).unwrap();
                    let t = syntax_tree.get_str(l).unwrap();
                    if re.is_match(&t) {
                        Some(l)
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some(loc) = loc {
                    SyntaxRuleResult::FailLocate(*loc)
                } else {
                    SyntaxRuleResult::Pass
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("action_block_with_side_effect")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Do not specify side effects within `assert` or `wait_order` action blocks.")
    }

    fn reason(&self) -> String {
        String::from("Side effects may cause undefined event ordering.")
    }
}
