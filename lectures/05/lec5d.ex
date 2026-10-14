# unquote fragments
defmodule M do
  for {k, v} <- [homer: 55, bart: 76] do
	def unquote(k)(), do: unquote(v)
  end
end

M.homer()
M.bart()
