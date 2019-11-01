use crate::linter::{Rule, RuleResult};
use sv_parser::{RefNode, StatementItem, StatementOrNull, SyntaxTree};

pub struct ForWithBegin;

impl Rule for ForWithBegin {
    fn check(&self, syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::LoopStatementFor(x) => {
                let (_, _, ref a) = x.nodes;

                let mut for_str = String::from("");
                syntax_tree.get_str(*x).map(|x| for_str.push_str(x));

                match a {
                    StatementOrNull::Statement(x) => {
                        let (_, _, ref x) = x.nodes;
                        match x {
                            StatementItem::SeqBlock(_) => (),
                            _ => {
                                if for_str.trim_end().contains("\n") {
                                    return RuleResult::Fail;
                                }
                            }
                        }
                    }
                    _ => (),
                }

                RuleResult::Pass
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("for_with_begin")
    }

    fn hint(&self) -> String {
        String::from("multiline `for` statement must have `begin`")
    }

    fn reason(&self) -> String {
        String::from("if there is not `begin`, the second statatement are confusing")
    }
}
