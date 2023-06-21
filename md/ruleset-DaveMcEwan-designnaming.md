
This ruleset only specifies the naming conventions preferred by user
 [DaveMcEwan](https://github.com/DaveMcEwan) for synthesisable design code.
These rules are also available in the superset **ruleset-DaveMcEwan-design**.


### Filesystem and Logical Hierarchy

In synthesisable design code, there are three main types of description
(package, module, and interface), which should normally be kept in separate
files for each description.
A straightforward way to manage these in a filesystem is to have the filename
match the identifier of the description inside, i.e. `myModule.sv` should
contain only the module named `myModule`, and `pkg1.sv` should contain
only the package named `pkg1`.
Note, this ruleset does not perform checks on file names.

Additionally, it is useful for the identifiers used in code to be immediately
obvious which type of description they refer to.
References to packages are always obvious because of the scope resolution
operator `::` (see IEEE1800-2017 clause 26.3).
However, interfaces and modules use identical instantiation syntax which makes
it difficult to easily identify if an instance refers to a module or interface
(see the definitions of `module_instantiation` and `interface_instantiation` in
IEEE1800-2017 Annex A.4 Instantiations).
A good naming scheme should make these easy to distinguish without introducing
too much visual noise.

The approach in this ruleset is similar to that in typical Haskell - the case
of the first letter of the identifier signifies what it refers to.
To begin, modules should have the first letter as Uppercase - Modules are the
most common thing to instance, so they should use the minimum number of
characters to avoid visual noise.
Next, packages are referred to more often than interfaces, so these are
distinguished by their first letter as lowercase.
Interface identifiers are usually used less often in a module than package
identifies - for example constants and functions in a package might be used
in the declarations and assignments of many signals, but interface identifiers
are only used for instantiations.
To distinguish instantiations of interfaces from modules, interface identifiers
should be prefixed with `ifc_`.
There are no restrictions on the rest of an interface identifier (everything
after the `ifc_` prefix) or modport or variable identifiers within an
interface declaration.

```toml
rules.lowercamelcase_package = true
rules.uppercamelcase_module = true
option.prefix_interface = "ifc_"
rules.prefix_interface = true
```

The above rules help readers to navigate a filesystem to find the right source
files containing packages, modules, and interfaces.
Another common situation where it is necessary to distinguish between these is
in examining tool output such as netlists and waveforms.
In these scenarios, naming conventions on hierarchical nodes can help engineers
distinguish between modules, interfaces, and generate blocks.
Although the LRM is clear about the implict naming of unlabelled generate
blocks (see IEEE1800-2017 clause 27.6), using a well-named label provides some
clarification about the intention behind that logic.
Instance identifiers of both modules and interfaces should be prefixed with
`u_`, whereas generate block labels should be prefixed with `l_`.

```toml
option.prefix_instance = "u_"
rules.prefix_instance = true
option.prefix_label = "l_"
rules.generate_case_with_label = true
rules.generate_for_with_label = true
rules.generate_if_with_label = true
```

A further convention, not checked by this ruleset, is to use Uppercase vs
lowercase for the first letter after the `u_` prefix to distinguish between
instances of modules vs interfaces.
For example, a module instance looks like `u_Foo` and an interface instance
looks like `u_foo`.
This makes it easier in navigate hierarchy in, for example, waveform viewers.

The above rules around filesystem and logical hierarchy are demonstrated in the
example below:

```systemverilog
/* filename: path/to/usb.sv */
package usb;                                    // package declaration
...
endpackage

////////////////////////////////////////////////////////////////////////////////

/* filename: path/to/ifc_fifo.sv */
interface ifc_fifo;                             // interface declaration
...
endinterface

////////////////////////////////////////////////////////////////////////////////

/* filename: path/to/UsbRx.sv */
module UsbRx                                    // module declaration
  ( ...
  , ifc_fifo.read                     rdData    // interface port
  , output var logic [usb::PID_W-1:0] o_pid     // package reference
  );
...
  ifc_fifo u_packer;                            // interface instance
...
  Fifo u_Queue ( ... );                         // module instance
...
  if (FOO) begin: l_foo                         // generate block
...
  end: l_foo
endmodule
```


### Ports and Direction

A distinctive feature of this naming convention is that all ports have prefix
denoting their direction: `i_`, `o_`, or `b_` for inputs, outputs, and
bi-directionals respectively.
This technique adds useful redundancy for readers/reviewers, which is
especially useful for very large modules.
By analogy, this is similar to the use of arrowheads in a electical schematic -
sure, arrowheads are not essential but they can be very useful in helping
readers understand the flow of information!
There are at least 6 ways in which this naming convention technique adds value:

- Visually highlight port connections.
  Internal signals should not have any prefix but ports should, so the prefixes
  make ports stand out clearly.
- Provide assurance that inputs are not accidentally connected to the wrong
  thing.
  For example, an input that should be connected directly to a DFF, but must
  not feed into any combinational logic.
- Clarify that the direction is the one which the author intended.
  For example, in `output var logic o_foo`, the direction is written twice
  (`output` keyword, then `o_` prefix).
  It isn't foolproof, but a mismatch such as `input o_foo` indicates a
  copy-pasta error which might be otherwise easily overlooked, especially
  because the rules about assignments to ports are not intuitive or
  consistently implemented across tools.
- In assertions (or other testbench code) where signals are connected via
  `bind`, assure readers that only inputs are `assume`d and that outputs are
  only `assert`d or `cover`ed (but outputs aren't `assume`d).
- In long files which don't fit on one screen, the reader doesn't need to
  scroll back-and-forth or memorize the portlist to determine which parts are
  connected to the boundary and which are purely internal.
- In complex synthesis flows, ports are often considered more stable API points
  than internal signals, so this naming convention highlights to script owners
  if they are using unstable points which might require more script
  maintenance.

Interface ports do not benefit in all of the same ways because `modport`s can
change the component signals' direction.
The only benefit which interface port prefixes would give is to highlight
connections to the module boundary vs internal interfaces.
This reason is deemed too weak to require the use of another prefix.

```toml
option.prefix_inout = "b_"
rules.prefix_inout = true
option.prefix_input = "i_"
rules.prefix_input = true
option.prefix_output = "o_"
rules.prefix_output = true
option.re_required_port_interface = "^[a-z]+[a-zA-Z0-9_]*$"
rules.re_required_port_interface = true
```

This illustrative example shows some of the common features addressed by this
convention:

```systemveilog
module Fifo
  ( input  var logic i_data   // Same name `data` used in both directions.
  , output var logic o_data

  , input  var logic i_push
  , input  var logic o_full   // Copy/paste error, now might be caught.

  , input  var logic i_pop
  , output var logic o_empty  // This looks better.

  , ifc_fifo.debug   dbg      // Interface port has no prefix.
  );

  ...

  RAMBLOCK u_ram
    ( RDATA   (o_data)        // Connected directly to port.
    , WDATA   (dataRetime_q)  // Connected to an internal DFF.
    );

  always_comb i_data = foo;   // Assignment to input looks wrong.
  always_comb o_full = i_pop && foo; // Feedthrough logic may be wrong.

  always_comb dbg.foo = foo;  // Direction of connection to interface
                              // port is not clear, regardless of the
                              // lack of prefix on `dbg`.
endmodule
```


### Elaboration-Time Constants

Functions may have a short lowercase name where the functionality is "obvious
and frequently used" but not provided in IEEE1800-2017 clause 20 Utility
system tasks and system functions, e.g. `flog2`.
More complex functions should have an `f_` prefix which can help readers
navigate a large codebase using text-based tools like `grep`.
Use good judgement to decide if a particular function is obvious and
frequently used.

Parameters, either overridable (`parameter`) or non-overridable (`localparam`)
must be fully UPPERCASE.
This helps clarify which portions of logic may be optimized during synthesis,
e.g. `assign foo = bar && MYCONST;` should be optimized to either
`assign foo = bar;` or `assign foo = 1'b0;`.

Generate loop variables (declared with `genvar`) should have lowercase names of
no more than 3 characters.
This rule aims to easily distinguish genvars from signals, allow the common
usage of single-character names like `i`, and prevent hard-to-read code with
long genvar names like `index_of element`.

```toml
option.re_required_function = "^([a-z]{1,1}[a-z0-9]{0,9}|f_[a-zA-Z0-9_]+)$"
rules.re_required_function = true
option.re_required_localparam = "^[A-Z]+[A-Z0-9_]*)$"
rules.re_required_localparam = true
option.re_required_parameter = "^[A-Z]+[A-Z0-9_]*)$"
rules.re_required_parameter = true
option.re_required_genvar = "^[a-z]{1,3}$"
rules.re_required_genvar = true
```

The above rules are shown in this example:

```systemveilog
module Buffer
  #(parameter int WIDTH = 8
  , localparam int N_STAGE = 5
  )
  ( ... );

  localparam bit MYBOOL = 1'b0;

  // Check that element of parameter `MYARRAY` satisfy constraints.
  function automatic bit [N_ITEM-1:0] f_paramcheck_MYARRAY ();
    for (int i=0; i < N_ITEM; i++)
      f_paramcheck_MYARRAY[i] =
        &{(0 <= MYARRAY[i])
        , (MYARRAY[i] < DEPTH)
        , (MYARRAY[i] != 2)
        , IS_ODD[i] ? (MYARRAY[i] % 2) : 1'b1
        };
  endfunction

  function automatic int unsigned flog2 (int x);
    ...
  endfunction

  for (genvar i = 0; i < N_ITEM; i++) begin: l_foo
    ...
  end: l_foo

  ...
endmodule
```


### Variables

Finally, some elements of design intent can be clarified by adding some useful
redundancy in the form of suffixes on identifiers, e.g. "Every signal which
should infer the output of a flip-flop with `_q`".
By using conventional terminology (`d` for input, `q` for output) readers
will be alerted to investigate any flip-flops (in a netlist) without this
suffix as the tools may not be treating the code as the original author
intended.

Some common suffixes include:

- `_d`: Input to a flip-flop.
- `_q`: Output from a flip-flop.
- `_lat`: Output from a latch.
- `_mem`: Memory model.
- `_a`: Asynchronous signal.
- `_n`: Active-low signal.
- `_dp`, `_dn`: Differential positive/negative pair.
- `_ana`: Analog signal.
- `_55MHz`: A signal with a required operating frequency.

Throughout this ruleset, prefixes are (usefully) redundant re-statements of
information already defined in SystemVerilog semantics, whereas suffixes are
(usefully) redundant clarifications of information which can only be inferred.
Note, svlint does not perform semantic analysis, so there are no rules to
check for these conventions.
