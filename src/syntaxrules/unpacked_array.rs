use crate::config::ConfigOption;
use crate::linter::{SyntaxRule, SyntaxRuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct UnpackedArray {
    under_localparam_decl: bool,
    under_param_decl: bool,
    under_specparam_decl: bool,
    under_inout_decl: bool,
    under_ansi_port_decl: bool,
    under_input_decl: bool,
    under_output_decl: bool,
    under_intf_port_decl: bool,
    under_ref_decl: bool,
    under_data_decl: bool,
    under_net_decl: bool,
}

impl SyntaxRule for UnpackedArray {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        option: &ConfigOption,
    ) -> SyntaxRuleResult {
        let node = match event {
            NodeEvent::Enter(x) => {
                match x {
                    RefNode::LocalParameterDeclaration(_) => {
                        self.under_localparam_decl = true;
                    }
                    RefNode::ParameterDeclaration(_) => {
                        self.under_param_decl = true;
                    }
                    RefNode::SpecparamDeclaration(_) => {
                        self.under_specparam_decl = true;
                    }
                    RefNode::InoutDeclaration(_) => {
                        self.under_inout_decl = true;
                    }
                    RefNode::AnsiPortDeclaration(_) => {
                        self.under_ansi_port_decl = true;
                    }
                    RefNode::InputDeclaration(_) => {
                        self.under_input_decl = true;
                    }
                    RefNode::OutputDeclaration(_) => {
                        self.under_output_decl = true;
                    }
                    RefNode::InterfacePortDeclaration(_) => {
                        self.under_intf_port_decl = true;
                    }
                    RefNode::RefDeclaration(_) => {
                        self.under_ref_decl = true;
                    }
                    RefNode::DataDeclaration(_) => {
                        self.under_data_decl = true;
                    }
                    RefNode::NetDeclaration(_) => {
                        self.under_net_decl = true;
                    }

                    _ => (),
                }
                x
            }
            NodeEvent::Leave(x) => {
                match x {
                    RefNode::LocalParameterDeclaration(_) => {
                        self.under_localparam_decl = false;
                    }
                    RefNode::ParameterDeclaration(_) => {
                        self.under_param_decl = false;
                    }
                    RefNode::SpecparamDeclaration(_) => {
                        self.under_specparam_decl = false;
                    }
                    RefNode::InoutDeclaration(_) => {
                        self.under_inout_decl = false;
                    }
                    RefNode::InputDeclaration(_) => {
                        self.under_input_decl = false;
                    }
                    RefNode::OutputDeclaration(_) => {
                        self.under_output_decl = false;
                    }
                    RefNode::InterfacePortDeclaration(_) => {
                        self.under_intf_port_decl = false;
                    }
                    RefNode::RefDeclaration(_) => {
                        self.under_ref_decl = false;
                    }
                    RefNode::DataDeclaration(_) => {
                        self.under_data_decl = false;
                    }
                    RefNode::NetDeclaration(_) => {
                        self.under_net_decl = false;
                    }

                    _ => (),
                }
                return SyntaxRuleResult::Pass;
            }
        };

        if let (true, RefNode::UnpackedDimension(_)) = (
            (self.under_localparam_decl && option.unpacked_array.localparam_declaration
                || self.under_param_decl && option.unpacked_array.param_declaration
                || self.under_specparam_decl && option.unpacked_array.specparam_declaration
                || self.under_inout_decl && option.unpacked_array.inout_declaration
                || self.under_ansi_port_decl && option.unpacked_array.ansi_port_declaration
                || self.under_input_decl && option.unpacked_array.input_declaration
                || self.under_output_decl && option.unpacked_array.output_declaration
                || self.under_intf_port_decl && option.unpacked_array.interface_port_declaration
                || self.under_ref_decl && option.unpacked_array.ref_declaration
                || self.under_data_decl && option.unpacked_array.data_declaration
                || self.under_net_decl && option.unpacked_array.net_declaration),
            node,
        ) {
            SyntaxRuleResult::Fail
        } else {
            SyntaxRuleResult::Pass
        }
    }
    fn name(&self) -> String {
        String::from("unpacked_array")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Avoid using unpacked dimensions in declarations.")
    }

    fn reason(&self) -> String {
        String::from(
            "Unpacked arrays are not guaranteed to be contiguous and can lead to synthesis issues.",
        )
    }
}
