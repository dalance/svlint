use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use regex::Regex;
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct StyleKeyword1Space {
    re: Option<Regex>,
}

impl Rule for StyleKeyword1Space {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        if self.re.is_none() {
            let keywords =
                [ "accept_on"
                , "alias"
                , "always"
                , "always_comb"
                , "always_ff"
                , "always_latch"
                , "and"
                , "assert"
                , "assign"
                , "assume"
                , "automatic"
                , "before"
                , "bind"
                , "bins"
                , "binsof"
                , "bit"
                , "buf"
                , "bufif0"
                , "bufif1"
                , "byte"
                , "case"
                , "casex"
                , "casez"
                , "cell"
                , "chandle"
                , "checker"
                , "class"
                , "clocking"
                , "cmos"
                , "config"
                , "const"
                , "constraint"
                , "context"
                , "cover"
                , "covergroup"
                , "coverpoint"
                , "cross"
                , "deassign"
                , "defparam"
                , "design"
                , "disable"
                , "dist"
                , "do"
                , "edge"
                , "enum"
                , "event"
                , "eventually"
                , "expect"
                , "export"
                , "extends"
                , "extern"
                , "first_match"
                , "for"
                , "force"
                , "foreach"
                , "forever"
                , "forkjoin"
                , "function"
                , "genvar"
                , "global"
                , "highz0"
                , "highz1"
                , "if"
                , "iff"
                , "ifnone"
                , "ignore_bins"
                , "illegal_bins"
                , "implements"
                , "implies"
                , "import"
                , "incdir"
                , "include"
                , "inside"
                , "instance"
                , "int"
                , "integer"
                , "interconnect"
                , "interface"
                , "intersect"
                , "large"
                , "let"
                , "liblist"
                , "library"
                , "local"
                , "localparam"
                , "logic"
                , "longint"
                , "macromodule"
                , "matches"
                , "medium"
                , "modport"
                , "module"
                , "nand"
                , "negedge"
                , "nettype"
                , "nexttime"
                , "nmos"
                , "nor"
                , "noshowcancelled"
                , "not"
                , "notif0"
                , "notif1"
                , "or"
                , "output"
                , "package"
                , "packed"
                , "parameter"
                , "pmos"
                , "posedge"
                , "primitive"
                , "priority"
                , "program"
                , "property"
                , "protected"
                , "pull0"
                , "pull1"
                , "pulldown"
                , "pullup"
                , "pulsestyle_ondetect"
                , "pulsestyle_onevent"
                , "pure"
                , "rand"
                , "randc"
                , "randcase"
                , "randsequence"
                , "rcmos"
                , "real"
                , "realtime"
                , "ref"
                , "reg"
                , "reject_on"
                , "release"
                , "repeat"
                , "restrict"
                , "rnmos"
                , "rpmos"
                , "rtran"
                , "rtranif0"
                , "rtranif1"
                , "s_always"
                , "s_eventually"
                , "s_nexttime"
                , "s_until"
                , "s_until_with"
                , "scalared"
                , "sequence"
                , "shortint"
                , "shortreal"
                , "showcancelled"
                , "signed"
                , "small"
                , "soft"
                , "solve"
                , "specparam"
                , "static"
                , "string"
                , "strong"
                , "strong0"
                , "strong1"
                , "struct"
                , "supply0"
                , "supply1"
                , "sync_accept_on"
                , "sync_reject_on"
                , "tagged"
                , "task"
                , "throughout"
                , "time"
                , "timeprecision"
                , "timeunit"
                , "tran"
                , "tranif0"
                , "tranif1"
                , "tri"
                , "tri0"
                , "tri1"
                , "triand"
                , "trior"
                , "trireg"
                , "type"
                , "typedef"
                , "union"
                , "unique"
                , "unique0"
                , "unsigned"
                , "until"
                , "until_with"
                , "untyped"
                , "use"
                , "uwire"
                , "var"
                , "vectored"
                , "virtual"
                , "void"
                , "wait"
                , "wait_order"
                , "wand"
                , "weak"
                , "weak0"
                , "weak1"
                , "while"
                , "wildcard"
                , "wire"
                , "with"
                , "within"
                , "wor"
                , "xnor"
                , "xor"
                ].join("|");

            /* Regex defines pattens which should fail.
            Keyword followed by something other than exactly 1 space:
                - nothing
                - 2 (or more) spaces
                - something not a space, except where keyword is a prefix of
                  another keyword (int/interface, always/always_ff)
            */
            self.re = Some(Regex::new(format!("^({})($|  |[^ _a-z01])", keywords).as_str()).unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };

        match node {
            RefNode::Keyword(x) => {
                let re = self.re.as_ref().unwrap();
                let kw = syntax_tree.get_str(*x).unwrap();

                if re.is_match(&kw) {
                    RuleResult::Fail
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("style_keyword_1space")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("keyword should be followed by a single space")
    }

    fn reason(&self) -> String {
        String::from("consistent style enhances readability")
    }
}
