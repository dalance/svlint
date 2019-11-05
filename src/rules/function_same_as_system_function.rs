use crate::linter::{Rule, RuleResult};
use sv_parser::{unwrap_node, RefNode, SyntaxTree};

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

impl Rule for FunctionSameAsSystemFunction {
    fn check(&self, syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::FunctionDeclaration(x) => {
                let a = unwrap_node!(x.clone(), FunctionIdentifier).unwrap();
                match a {
                    RefNode::FunctionIdentifier(a) => {
                        let a = syntax_tree.get_str(a).unwrap();
                        if SYSTEM_FUNCTION.contains(&a) {
                            RuleResult::Fail
                        } else {
                            RuleResult::Pass
                        }
                    }
                    _ => unreachable!(),
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("function_same_as_system_function")
    }

    fn hint(&self) -> String {
        String::from("the name of `function` must not be the same as system function")
    }

    fn reason(&self) -> String {
        String::from("some tools confuse function with system function")
    }
}
