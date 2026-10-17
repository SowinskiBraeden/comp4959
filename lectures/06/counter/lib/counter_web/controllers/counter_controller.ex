defmodule CounterWeb.CounterController do
  use CounterWeb, :controller

  def new(conn, %{"name" => name}) do
	Counter.DynamicSupervisor.start_worker(name)
	render(conn, :new)
  end

  def value(conn, %{"name" => name}) do
	value = Counter.Worker.value(name)
	render(conn, :value, value: value)
  end

  def inc(conn, %{"name" => name, "amt" => amt}) when is_integer(amt) do
	Counter.Worker.inc(name, amt)
	render(conn, :inc)
  end
end
