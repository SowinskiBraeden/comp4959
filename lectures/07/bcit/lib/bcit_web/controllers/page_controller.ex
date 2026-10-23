defmodule BcitWeb.PageController do
  use BcitWeb, :controller

  def home(conn, _params) do
    render(conn, :home)
  end
end
