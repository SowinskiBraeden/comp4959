defmodule CounterWeb.CounterJSON do
  def value(assigns) do
	%{value: assigns.value}
  end

  def new(_assigns) do
	%{status: "ok"}
  end

  def inc(_assigns) do
	%{status: "ok"}
  end
end
