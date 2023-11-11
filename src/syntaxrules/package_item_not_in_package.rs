use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{NodeEvent, PackageItem, RefNode, SyntaxTree};

#[derive(Default)]
pub struct PackageItemNotUnderPackage {
    under_package_declaration: bool,
}

impl SyntaxRule for PackageItemNotUnderPackage {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> SyntaxRuleResult {
        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::PackageDeclaration(_) => {
                        self.under_package_declaration = true;
                    }
                    _ => ()
                }
                x
            }
            NodeEvent::Leave(x) => {
                match x {
                    RefNode::PackageDeclaration(_) => {
                        self.under_package_declaration = false;
                    }
                    _ => ()
                }
                return SyntaxRuleResult::Pass;
            }
        };

        if self.under_package_declaration {
            SyntaxRuleResult::Pass
        } else {
            match node {
                RefNode::PackageItem(PackageItem::PackageOrGenerateItemDeclaration(_)) |
                RefNode::PackageItem(PackageItem::PackageExportDeclaration(_)) => SyntaxRuleResult::Fail,
                _ => SyntaxRuleResult::Pass
            }
        }
    }

    fn name(&self) -> String {
        String::from("package_item_not_in_package")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Place item into a package, module, interface, program, udp, or config.")
    }

    fn reason(&self) -> String {
        String::from("Globally-scoped items are not supported by some tools.")
    }
}
