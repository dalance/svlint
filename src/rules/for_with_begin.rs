use crate::linter::{Rule, RuleResult};
use sv_parser::{NodeEvent, RefNode, StatementItem, StatementOrNull, SyntaxTree};

#[derive(Default)]
pub struct ForWithBegin;

impl Rule for ForWithBegin {
    fn check(&mut self, syntax_tree: &SyntaxTree, event: &NodeEvent) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Skip;
            }
        };
        match node {
            RefNode::LoopStatementFor(x) => {
                let (_, _, ref a) = x.nodes;

                let mut for_str = String::from("");
                syntax_tree.get_str(*x).map(|x| for_str.push_str(x));

                if let StatementOrNull::Statement(x) = a {
                    let (_, _, ref x) = x.nodes;
                    match x {
                        StatementItem::SeqBlock(_) => (),
                        _ => {
                            if for_str.trim_end().contains('\n') {
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

    fn hint(&self) -> String {
        String::from("multiline `for` statement must have `begin`")
    }

    fn reason(&self) -> String {
        String::from("if there is not `begin`, the second statatement are confusing")
    }
}
