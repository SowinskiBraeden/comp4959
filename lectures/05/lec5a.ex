defmodule M do
	def if_fun(predicate, do: do_block, else: else_block) do
		cond do
			predicate -> do_block
			true -> else_block
		end
	end

	defmacro inspect(x) do
		IO.inspect(x)
	end

	defmacro if_macro(predicate, do: do_block, else: else_block) do
		quote do
			cond do
				unquote(predicate) -> unquote(do_block)
				true -> unquote(else_block)
			end
		end
	end

	defmacro while(condition, do: do_block) do
		quote do
			try do
				for _ <- Stream.cycle [:ok] do
					if unquote(condition), do: unquote(do_block),
					else: throw :break
				end
			catch :throw, :break -> :ok end
		end
	end
end
