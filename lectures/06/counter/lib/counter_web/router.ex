defmodule CounterWeb.Router do
  use CounterWeb, :router

  pipeline :api do
	plug :accepts, ["json"]
  end

  scope "/api", CounterWeb do
	pipe_through :api

	post "/counter/new/:name", CounterController, :new
	get "/counter/value/:name", CounterController, :value
	post "/counter/inc/:name", CounterController, :inc
  end

  # Enable LiveDashboard in development
  if Application.compile_env(:counter, :dev_routes) do
	# If you want to use the LiveDashboard in production, you should put
	# it behind authentication and allow only admins to access it.
	# If your application does not have an admins-only section yet,
	# you can use Plug.BasicAuth to set up some basic authentication
	# as long as you are also using SSL (which you should anyway).
	import Phoenix.LiveDashboard.Router

	scope "/dev" do
	  pipe_through [:fetch_session, :protect_from_forgery]

	  live_dashboard "/dashboard", metrics: CounterWeb.Telemetry
	end
  end
end
