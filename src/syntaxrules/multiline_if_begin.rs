use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_locate, NodeEvent, RefNode, StatementItem, StatementOrNull, SyntaxTree};

#[derive(Default)]
pub struct MultilineIfBegin;

impl Rule for MultilineIfBegin {
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
            RefNode::ConditionalStatement(x) => {
                let (ref a, ref b, ref c, ref d, ref e, ref f) = x.nodes;

                // if statement
                let mut if_str = String::from("");
                syntax_tree.get_str(a).map(|x| if_str.push_str(x));
                syntax_tree.get_str(b).map(|x| if_str.push_str(x));
                syntax_tree.get_str(c).map(|x| if_str.push_str(x));
                syntax_tree.get_str_trim(d).map(|x| if_str.push_str(x));

                if let StatementOrNull::Statement(x) = d {
                    let (_, _, ref x) = x.nodes;
                    match x {
                        StatementItem::SeqBlock(_) => (),
                        _ => {
                            if if_str.contains('\n') {
                                return RuleResult::Fail;
                            }
                        }
                    }
                }

                // else if statement
                for e in e {
                    let (ref a, ref b, ref c, ref d) = e;

                    let mut elsif_str = String::from("");
                    syntax_tree.get_str(a).map(|x| elsif_str.push_str(x));
                    syntax_tree.get_str(b).map(|x| elsif_str.push_str(x));
                    syntax_tree.get_str(c).map(|x| elsif_str.push_str(x));
                    syntax_tree.get_str_trim(d).map(|x| elsif_str.push_str(x));

                    if let StatementOrNull::Statement(x) = d {
                        let (_, _, ref x) = x.nodes;
                        match x {
                            StatementItem::SeqBlock(_) => (),
                            _ => {
                                if elsif_str.contains('\n') {
                                    let locate = unwrap_locate!(a).unwrap();
                                    return RuleResult::FailLocate(*locate);
                                }
                            }
                        }
                    };
                }

                // else statement
                if let Some(f) = f {
                    let (ref a, ref b) = f;

                    let mut else_str = String::from("");
                    syntax_tree.get_str(a).map(|x| else_str.push_str(x));
                    syntax_tree.get_str_trim(b).map(|x| else_str.push_str(x));

                    if let StatementOrNull::Statement(x) = b {
                        let (_, _, ref x) = x.nodes;
                        match x {
                            StatementItem::SeqBlock(_) => (),
                            _ => {
                                if else_str.contains('\n') {
                                    let locate = unwrap_locate!(a).unwrap();
                                    return RuleResult::FailLocate(*locate);
                                }
                            }
                        }
                    };
                }

                RuleResult::Pass
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("multiline_if_begin")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Add `begin`/`end` around multi-line `if` statement.")
    }

    fn reason(&self) -> String {
        String::from("Without `begin`/`end`, the conditional statement may be confusing.")
    }
}
