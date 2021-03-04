use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct DefaultNettypeNone;

impl Rule for DefaultNettypeNone {
    fn check(&mut self, syntax_tree: &SyntaxTree, event: &NodeEvent) -> RuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };

        match node {
            RefNode::SourceText(x) => {
                // get the top comments of source code
                let (comments, _, _) = &x.nodes;

                for comment in comments {
                    let default_nettype = unwrap_node!(comment, DefaultNettypeCompilerDirective);
                    match default_nettype {
                        Some(RefNode::DefaultNettypeCompilerDirective(x)) => {
                            let (_, _, x) = &x.nodes;
                            let mut nettype = String::from("");
                            syntax_tree.get_str_trim(x).map(|x| nettype.push_str(x));

                            if nettype == "none" {
                                return RuleResult::Pass;
                            }
                        }
                        _ => (),
                    }
                }

                RuleResult::Fail
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("default_nettype_none")
    }

    fn hint(&self) -> String {
        String::from("`` `default_nettype none`` should be at the top of source code")
    }

    fn reason(&self) -> String {
        String::from("`` `default_nettype none`` can detect unintentional implicit wires")
    }
}
