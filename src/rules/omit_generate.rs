use crate::linter::Rule;
use sv_parser::RefNode;

pub struct OmitGenerate;

impl Rule for OmitGenerate {
    fn check(&self, node: &RefNode) -> bool {
        match node {
            RefNode::GenerateRegion(_) => false,
            _ => true,
        }
    }

    fn name(&self) -> String {
        String::from("omit generate")
    }

    fn hint(&self) -> String {
        String::from("'generate'/'endgenerate' must be omitted")
    }
}
