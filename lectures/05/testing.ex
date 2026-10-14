defmodule Testing do
  defmacro check(predicate) do
	test = Macro.to_string(predicate)
	quote do
	  cond do
		unquote(predicate) -> IO.puts("passed ... #{unquote(test)}")
		true -> IO.puts("FAILED ... #{unquote(test)}")
	  end
	end
  end

  defmacro test(description, do: block) do
	name = "test_#{description}"
	fn_name = String.to_atom(name)
	quote do
	  @tests [unquote(fn_name) | @tests]
	  def unquote(fn_name)() do
		IO.puts(unquote(name) <> ":")
		unquote(block)
	  end
	end
  end

  defmacro __before_compile__(_env) do
  # defmacro make_run_tests() do
	quote do
	  def run_tests() do
		Enum.each(Enum.reverse(@tests), fn f -> apply(__MODULE__, f, []) end)
	  end
	end
  end

  defmacro __using__(_opts) do
	quote do
	  import unquote(__MODULE__)
	  @tests []
	  @before_compile unquote(__MODULE__)
	end
  end
end

defmodule Tests do
  use Testing # this requires Testing and call Testing.__using__

  test "equality" do
	check(1 + 1 == 2)
	check(1 + 2 == 4)
  end

  test "greater" do
	check(1 > 2)
	check(1 < 2)
  end

  # make_run_tests()
end
