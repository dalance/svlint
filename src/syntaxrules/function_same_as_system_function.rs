use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{unwrap_node, NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct FunctionSameAsSystemFunction;

const SYSTEM_FUNCTION: &[&str] = &[
    "finish",
    "stop",
    "exit",
    "realtime",
    "stime",
    "time",
    "printtimescale",
    "timeformat",
    "bitstoreal",
    "realtobits",
    "bitstoshortreal",
    "shortrealtobits",
    "itor",
    "rtoi",
    "signed",
    "unsigned",
    "cast",
    "bits",
    "isunbounded",
    "typename",
    "unpacked_dimensions",
    "dimensions",
    "left",
    "right",
    "low",
    "high",
    "increment",
    "size",
    "clog2",
    "asin",
    "ln",
    "acos",
    "log10",
    "atan",
    "exp",
    "atan2",
    "sqrt",
    "hypot",
    "pow",
    "sinh",
    "floor",
    "cosh",
    "ceil",
    "tanh",
    "sin",
    "asinh",
    "cos",
    "acosh",
    "tan",
    "atanh",
    "countbits",
    "countones",
    "onehot",
    "onehot0",
    "isunknown",
    "fatal",
    "error",
    "warning",
    "info",
    "fatal",
    "error",
    "warning",
    "info",
    "asserton",
    "assertoff",
    "assertkill",
    "assertcontrol",
    "assertpasson",
    "assertpassoff",
    "assertfailon",
    "assertfailoff",
    "assertnonvacuouson",
    "assertvacuousoff",
    "sampled",
    "rose",
    "fell",
    "stable",
    "changed",
    "past",
    "past_gclk",
    "rose_gclk",
    "fell_gclk",
    "stable_gclk",
    "changed_gclk",
    "future_gclk",
    "rising_gclk",
    "falling_gclk",
    "steady_gclk",
    "changing_gclk",
    "coverage_control",
    "coverage_get_max",
    "coverage_get",
    "coverage_merge",
    "coverage_save",
    "get_coverage",
    "set_coverage_db_name",
    "load_coverage_db",
    "random",
    "dist_chi_square",
    "dist_erlang",
    "dist_exponential",
    "dist_normal",
    "dist_poisson",
    "dist_t",
    "dist_uniform",
    "q_initialize",
    "q_add",
    "q_remove",
    "q_full",
    "q_exam",
    "async$and$array",
    "async$and$plane",
    "async$nand$array",
    "async$nand$plane",
    "async$or$array",
    "async$or$plane",
    "async$nor$array",
    "async$nor$plane",
    "sync$and$array",
    "sync$and$plane",
    "sync$nand$array",
    "sync$nand$plane",
    "sync$or$array",
    "sync$or$plane",
    "sync$nor$array",
    "sync$nor$plane",
    "system",
];

impl SyntaxRule for FunctionSameAsSystemFunction {
    fn check(
        &mut self,
        syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> SyntaxRuleResult {
        let node = match event {
            NodeEvent::Enter(x) => x,
            NodeEvent::Leave(_) => {
                return SyntaxRuleResult::Pass;
            }
        };
        match node {
            RefNode::FunctionDeclaration(x) => {
                let a = unwrap_node!(*x, FunctionIdentifier).unwrap();
                match a {
                    RefNode::FunctionIdentifier(a) => {
                        let a = syntax_tree.get_str(a).unwrap();
                        if SYSTEM_FUNCTION.contains(&a) {
                            SyntaxRuleResult::Fail
                        } else {
                            SyntaxRuleResult::Pass
                        }
                    }
                    _ => unreachable!(),
                }
            }
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("function_same_as_system_function")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Rename `function` to something other than the name of a built-in function.")
    }

    fn reason(&self) -> String {
        String::from("Name clashes may cause confusion amongst tools and readers.")
    }
}
