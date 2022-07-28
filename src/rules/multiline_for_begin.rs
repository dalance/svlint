use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{NodeEvent, RefNode, StatementItem, StatementOrNull, SyntaxTree};
use indoc::indoc;

#[derive(Default)]
pub struct MultilineForBegin;

impl Rule for MultilineForBegin {
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
        String::from("multiline_for_begin")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Add `begin`/`end` around multi-line `for` statement.")
    }

    fn reason(&self) -> String {
        String::from("Without `begin`/`end`, the loop statement may be confusing.")
    }

    fn explanation(&self) -> String {
        String::from(indoc!{"
        This rule is to help prevent a common class of coding mistake, where a future
        maintainer attempts to add further statements to the loop, but accidentally
        writes something different.

        See also:
          - **multiline_if_begin** - Useful companion rule.
          - **style_indent** - Useful companion rule.

        The most relevant clauses of IEEE1800-2017 are:
          - 12.7 Loop statements
        "})
    }
}
