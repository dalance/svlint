use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{NodeEvent, RefNode, StatementItem, StatementOrNull, SyntaxTree};

#[derive(Default)]
pub struct ForWithBegin {
    disable: bool,
}

impl Rule for ForWithBegin {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };
        match node {
            RefNode::LoopStatementFor(x) => {
                let (_, _, ref a) = x.nodes;

                let mut for_str = String::from("");
                syntax_tree.get_str_trim(*x).map(|x| for_str.push_str(x));

                if let StatementOrNull::Statement(x) = a {
                    let (_, _, ref x) = x.nodes;
                    match x {
                        StatementItem::SeqBlock(_) => (),
                        _ => {
                            if for_str.contains('\n') {
                                return RuleResult::Fail;
                            }
                        }
                    }
                }

                RuleResult::Pass
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("for_with_begin")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("multiline `for` statement must have `begin`")
    }

    fn reason(&self) -> String {
        String::from("if there is not `begin`, the second statement are confusing")
    }

    fn disabled(&mut self, disable: Option<bool>) -> bool {
        match disable {
            Some(x) => {
                self.disable = x;
            }
            _ => {}
        }
        self.disable
    }
}
