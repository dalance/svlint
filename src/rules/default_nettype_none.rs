use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};
use indoc::indoc;

#[derive(Default)]
pub struct DefaultNettypeNone;

impl Rule for DefaultNettypeNone {
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

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Place `` `default_nettype none`` at the top of source code.")
    }

    fn reason(&self) -> String {
        String::from("Compiler directive `` `default_nettype none`` detects unintentional implicit wires.")
    }

    fn explanation(&self) -> String {
        String::from(indoc!{"
        The `` `default_netype`` compiler directive can be used to specify the net type
        of implicit nets, i.e. where a signal is referenced, or assigned to, without
        being declared.
        IEEE1800-2017 clause 22.8 stipulates \"When no `` `default_nettype`` directive
        is present or if the `` `resetall`` directive is specified, implicit nets are of
        type `wire`.\"

        SystemVerilog makes a distiction between variables (only 0 or 1 drivers)
        and nets (0 or more drivers).
        IEEE1364-2001 (Verilog) uses variables as abstractions for data storage
        elements (`reg`, `integer`, `real`, `time`, `realtime`).
        In contrast, IEEE1800-2017 (SystemVerilog) the distinction between nets and
        variables is defined by how a simulator must calculate a value.
        In a simulator, a variable stores a value, but a net's value is calculated by
        evaluating the strength of all drivers.
        To keep compatibility with Verilog, the default net type of an undeclared net
        in SystemVerilog is `wire` (a net, not a variable), which requires evaluating a
        list of values with strengths, rather than simply looking up a value.
        The distiction between data storage elements and physical wires is therefore
        made in using `always_comb`, `always_ff`, and (less commonly) `always_latch`
        keywords.

        Variables are preferred over nets for most digital logic for 2 reasons:
          - Only 0 or 1 drivers allowed, so an accidental multi-driving is caught by
            a compile time error.
          - Simulator performance (dependent on implemetation).
            Value can be found by lookup, rather than evaluation of drivers.
        When `` `default_nettype none`` is used, all signals must be declared, thus
        forcing the author to consider whether they mean a variable or a net.

        The most relevant clauses of IEEE1800-2017 are:
          - 6.5 Nets and variables
          - 22.8 default nettype

        Note: One prominent paper (Cliff Cummings, HDLCON 2002) recommends *against*
        using `` `default_nettype none`` on the basis that concise, typeless code has
        fewer opportunities for mistakes.
        This attitude was popular at the time, e.g. Python's dynamic typing, but
        modern attitudes are now favouring explicit types, e.g. Python's new type
        checking syntax and tooling.
        Additionally, the reasoning behind this guideline only applies principally to
        IEEE1364, but not strongly to IEEE1800.
        "})
    }
}
