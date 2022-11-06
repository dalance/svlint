use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct FunctionWithAutomatic {
    lifetimes: Vec<Lifetime>,
}

enum Lifetime {
    Static,
    Automatic,
}

impl Rule for FunctionWithAutomatic {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _option: &ConfigOption,
    ) -> RuleResult {
        match event {
            NodeEvent::Enter(RefNode::ModuleAnsiHeader(x)) => {
                let (_, _, ref a, _, _, _, _, _) = x.nodes;
                match a {
                    Some(sv_parser::Lifetime::Automatic(_)) => {
                        self.lifetimes.push(Lifetime::Automatic)
                    }
                    Some(sv_parser::Lifetime::Static(_)) => self.lifetimes.push(Lifetime::Static),
                    _ => (),
                }
                RuleResult::Pass
            }
            NodeEvent::Enter(RefNode::ModuleNonansiHeader(x)) => {
                let (_, _, ref a, _, _, _, _, _) = x.nodes;
                match a {
                    Some(sv_parser::Lifetime::Automatic(_)) => {
                        self.lifetimes.push(Lifetime::Automatic)
                    }
                    Some(sv_parser::Lifetime::Static(_)) => self.lifetimes.push(Lifetime::Static),
                    _ => (),
                }
                RuleResult::Pass
            }
            NodeEvent::Enter(RefNode::InterfaceAnsiHeader(x)) => {
                let (_, _, ref a, _, _, _, _, _) = x.nodes;
                match a {
                    Some(sv_parser::Lifetime::Automatic(_)) => {
                        self.lifetimes.push(Lifetime::Automatic)
                    }
                    Some(sv_parser::Lifetime::Static(_)) => self.lifetimes.push(Lifetime::Static),
                    _ => (),
                }
                RuleResult::Pass
            }
            NodeEvent::Enter(RefNode::InterfaceNonansiHeader(x)) => {
                let (_, _, ref a, _, _, _, _, _) = x.nodes;
                match a {
                    Some(sv_parser::Lifetime::Automatic(_)) => {
                        self.lifetimes.push(Lifetime::Automatic)
                    }
                    Some(sv_parser::Lifetime::Static(_)) => self.lifetimes.push(Lifetime::Static),
                    _ => (),
                }
                RuleResult::Pass
            }
            NodeEvent::Enter(RefNode::ProgramAnsiHeader(x)) => {
                let (_, _, ref a, _, _, _, _, _) = x.nodes;
                match a {
                    Some(sv_parser::Lifetime::Automatic(_)) => {
                        self.lifetimes.push(Lifetime::Automatic)
                    }
                    Some(sv_parser::Lifetime::Static(_)) => self.lifetimes.push(Lifetime::Static),
                    _ => (),
                }
                RuleResult::Pass
            }
            NodeEvent::Enter(RefNode::ProgramNonansiHeader(x)) => {
                let (_, _, ref a, _, _, _, _, _) = x.nodes;
                match a {
                    Some(sv_parser::Lifetime::Automatic(_)) => {
                        self.lifetimes.push(Lifetime::Automatic)
                    }
                    Some(sv_parser::Lifetime::Static(_)) => self.lifetimes.push(Lifetime::Static),
                    _ => (),
                }
                RuleResult::Pass
            }
            NodeEvent::Enter(RefNode::PackageDeclaration(x)) => {
                let (_, _, ref a, _, _, _, _, _, _) = x.nodes;
                match a {
                    Some(sv_parser::Lifetime::Automatic(_)) => {
                        self.lifetimes.push(Lifetime::Automatic)
                    }
                    Some(sv_parser::Lifetime::Static(_)) => self.lifetimes.push(Lifetime::Static),
                    _ => (),
                }
                RuleResult::Pass
            }
            NodeEvent::Enter(RefNode::ClassDeclaration(_)) => {
                self.lifetimes.push(Lifetime::Automatic);
                RuleResult::Pass
            }
            NodeEvent::Enter(RefNode::FunctionDeclaration(x)) => {
                let (_, ref a, _) = x.nodes;
                match a {
                    Some(sv_parser::Lifetime::Automatic(_)) => RuleResult::Pass,
                    Some(sv_parser::Lifetime::Static(_)) => RuleResult::Fail,
                    None => {
                        if let Some(x) = self.lifetimes.last() {
                            match x {
                                Lifetime::Automatic => RuleResult::Pass,
                                Lifetime::Static => RuleResult::Fail,
                            }
                        } else {
                            RuleResult::Fail
                        }
                    }
                }
            }
            NodeEvent::Leave(RefNode::ModuleDeclarationAnsi(_))
            | NodeEvent::Leave(RefNode::ModuleDeclarationNonansi(_))
            | NodeEvent::Leave(RefNode::InterfaceDeclarationAnsi(_))
            | NodeEvent::Leave(RefNode::InterfaceDeclarationNonansi(_))
            | NodeEvent::Leave(RefNode::ProgramDeclarationAnsi(_))
            | NodeEvent::Leave(RefNode::ProgramDeclarationNonansi(_))
            | NodeEvent::Leave(RefNode::PackageDeclaration(_))
            | NodeEvent::Leave(RefNode::ClassDeclaration(_)) => {
                self.lifetimes.pop();
                RuleResult::Pass
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("function_with_automatic")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Add the `automatic` lifetime specifier to function.")
    }

    fn reason(&self) -> String {
        String::from("Static lifetime of function items causes a simulation/synthesis mismatch.")
    }
}
