use crate::config::ConfigOption;
use crate::linter::{Rule, RuleResult};
use sv_parser::{NodeEvent, RefNode, SyntaxTree};

#[derive(Default)]
pub struct StyleCommaleading;

impl Rule for StyleCommaleading {
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

            // TODO: Items from Annex A
            // - checker_declaration (checker_port_list)
            // - parameter_port_list
            // - list_of_ports
            // - port_expression
            // - class_constructor_prototype (tf_port_list)
            // - class_constructor_declaration (tf_port_list)
            // - class_constructor_declaration (list_of_arguments)
            // - class_new (list_of_arguments)
            // - function_body_declaration (tf_port_list)
            // - function_prototype (tf_port_list)
            // - task_body_declaration (tf_port_list)
            // - modport_item (modport_ports_declaration)
            // - property_instance (property_list_of_arguments)
            // - property_declaration (property_port_list)
            // - sequence_declaration (sequence_port_list)
            // - sequence_instance (sequence_list_of_arguments)
            // - select_condition (covergroup_range_list)
            // - parameter_value_assignment (list_of_parameter_assignements)
            // - hierarchical_instance (list_of_port_assignements)
            // - checker_instantiation (list_of_checker_port_assignements)
            // - udp_nonansi_declaration (udp_port_list)
            // - udp_ansi_declaration (udp_declaration_port_list)
            // - udp_instance
            // - wait_statement (wait_order)
            // - pattern
            // - assignment_pattern
            // - assignment_pattern_net_lvalue
            // - assignment_pattern_variable_lvalue
            // - concatenation
            // - constant_concatenation
            // - constant_multiple_concatenation
            // - module_path_concatenation
            // - module_path_multiple_concatenation
            // - stream_concatenation
            // - module_path_concatenation
            // - tf_call (list_of_arguments)
            // - system_tf_call (list_of_arguments)
            // - method_call_body (list_of_arguments)
            // - array_manipulation_call (list_of_arguments)
            // - inside_expression (open_range_list)
            // - net_lvalue
            // - variable_lvalue
            // TODO: t = text including parens/braces
            // if t contains newline, then opening paren/brace followed by 1space
            // else opening paren/brace followed by nothing

            RefNode::Symbol(x) => {
                let t = syntax_tree.get_str(*x).unwrap();
                if t.starts_with(",") && t != ", " {
                    RuleResult::Fail
                } else {
                    RuleResult::Pass
                }
            }
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("style_commasleading")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from(format!(
            "comma-separated lists should be in comma-leading format"
        ))
    }

    fn reason(&self) -> String {
        String::from("Naming convention simplifies audit.")
    }
}
