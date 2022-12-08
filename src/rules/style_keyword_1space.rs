use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use regex::Regex;
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct StyleKeyword1Space {
    re_split: Option<Regex>,
    re_kw: Option<Regex>,
    re_succ: Option<Regex>,
}

impl Rule for StyleKeyword1Space {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        /*
        re_split extracts keyword from anything following it.
        re_kw is used to selectively apply this rule to specific keywords.
        re_succ matches what is allowed after the keyword.
            - exactly 1space, then nothing
        */
        if self.re_split.is_none() {
            self.re_split = Some(Regex::new(r"(?P<kw>[\\$'BXZa-z_01]+)(?P<succ>(?s:.)*)").unwrap());
        }
        if self.re_kw.is_none() {
            let keywords =
                [ "accept_on" // {{{
                , "alias"
                , "always"
                , "always_ff"
                , "and"
                , "assert"
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
                , "case"
                , "casex"
                , "casez"
                , "cell"
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
                , "interconnect"
                , "interface"
                , "intersect"
                , "large"
                , "let"
                , "liblist"
                , "library"
                , "local"
                , "localparam"
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
                , "showcancelled"
                , "small"
                , "soft"
                , "solve"
                , "specparam"
                , "static"
                , "strong"
                , "strong0"
                , "strong1"
                , "struct"
                , "sync_accept_on"
                , "sync_reject_on"
                , "tagged"
                , "task"
                , "throughout"
                , "timeprecision"
                , "timeunit"
                , "tran"
                , "tranif0"
                , "tranif1"
                , "trireg"
                , "type"
                , "typedef"
                , "union"
                , "unique"
                , "unique0"
                , "until"
                , "until_with"
                , "untyped"
                , "use"
                , "var"
                , "vectored"
                , "virtual"
                , "wait"
                , "wait_order"
                , "weak"
                , "weak0"
                , "weak1"
                , "while"
                , "wildcard"
                , "with"
                , "within"
                , "xnor"
                , "xor"
                ].join("|"); // }}}

            self.re_kw = Some(Regex::new(format!("^({})$", keywords).as_str()).unwrap());
        }
        if self.re_succ.is_none() {
            self.re_succ = Some(Regex::new(r"^ $").unwrap());
        }

        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return RuleResult::Pass;
            }
        };

        match node {
            RefNode::Keyword(x) => {
                let re_split = self.re_split.as_ref().unwrap();
                let re_kw = self.re_kw.as_ref().unwrap();
                let t = syntax_tree.get_str(*x).unwrap();
                let caps = re_split.captures(&t).unwrap();

                if re_kw.is_match(&caps[1]) {
                    let re_succ = self.re_succ.as_ref().unwrap();

                    if re_succ.is_match(&caps[2]) {
                        RuleResult::Pass
                    } else {
                        RuleResult::Fail
                    }
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
        String::from("Follow keyword with exactly 1 space.")
    }

    fn reason(&self) -> String {
        String::from("Consistent use of whitespace enhances readability by reducing visual noise.")
    }
}
