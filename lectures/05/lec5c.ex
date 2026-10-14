defmodule M do
  defmacro say(msg) do
	name = String.to_atom("say_#{msg}")
	quote do
	  def name() do
		IO.puts(unquote(msg))
	  end
	end
  end
end

defmodule Test do
  import M

  say("hello")
end
