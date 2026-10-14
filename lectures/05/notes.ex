# see abstract syntax tree (ast)
quote do: 1 + 2
# {:+, [context: Elixir, imports: [{1, Kernel}, {2, Kernel}]], [1, 2]}
#
#	   +
#	  / \
#	 1	 2
#

alias String, as: S
# {:__aliases__, [alias: String], [:S]}

### Abstract literal
quote do: 1 # -> 1
quote do: "hello" # -> "hello"
quote do: [1, 2, 3] # -> [1, 2, 3] # a list of ast literals is an ast literal
quote do: {1, 2} # -> {1, 2} # doubles are ast literals but not triples


### Macro - something that takes an AST and returns an AST
x = 2
quote do: x # -> {:x, [], Elixir} # doesnt evaluate it, just ast of expression
quote do: unquote(x) # -> 2
