This rule forbids unpacked array declarations.

Unpacked arrays are not guaranteed to be represented as contiguous memory, and
can cause issues with synthesis tools, especially with how multidimensional
arrays are synthesized. For example, a synthesis tool might synthesize out
unused memory locations of an unpacked array which is not the intended behavior.

Additionally, packed arrays allow the user to intuitively index and slice the
array and apply bitwise operations.

This rule by default targets data declarations, but can be configured to target
other declarations. To target a declaration, enable the corresponding boolean
option in the configuration file.

``` toml
[option.unpacked_array]
localparam_declaration  = false
param_declaration  = false
specparam_declaration  = false
inout_declaration  = false
ansi_port_declaration  = false
input_declaration  = false
output_declaration  = false
intf_port_declaration  = false
ref_declaration  = false
data_declaration  = true # enabled by default
net_declaration = false
```

The most relevant clauses of IEEE1800-2017 are:

- 7.4 Packed and unpacked arrays
