defmodule M do
  defmacro context() do
	# macro's context
	IO.inspect(__MODULE__)
	quote do
	  # caller's context
	  IO.inspect(unquote(__MODULE__))
	  IO.inspect(__MODULE__)
	end
  end

  defmacro twice(x) do
	quote do
	  unquote(x) + unquote(x)
	end
  end

  defmacro twice_v2(x) do
	quote bind_quoted: [x: x] do
	  x + x
	end
  end
end

defmodule Test do
  import M

  def test() do
	context()
  end
end
