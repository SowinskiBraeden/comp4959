defmodule HelloWeb.HelloController do
  use HelloWeb, :controller

  def index(conn, _params) do
	IO.inspect(conn)
	render(conn, :index)
  end

  # match map
  def from(conn, %{"from" => from}) do
	IO.inspect(conn)
	render(conn, :from, from: from)
  end
end
