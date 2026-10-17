defmodule Counter.Application do
  # See https://elixir.hexdocs.pm/Application.html
  # for more information on OTP Applications
  @moduledoc false
  @table Counter.Table
  @registry Counter.Registry

  use Application

  @impl true
  def start(_type, _args) do
	:ets.new(@table, [:set, :public, :named_table])

	children = [
	  CounterWeb.Telemetry,
	  {DNSCluster, query: Application.get_env(:counter, :dns_cluster_query) || :ignore},
	  {Phoenix.PubSub, name: Counter.PubSub},
	  {Registry, name: @registry, keys: :unique},
	  # Starts workers on demand by calling: Counter.DynamicSupervisor.start_worker(name)
	  Counter.DynamicSupervisor,
	  # Start to serve requests, typically the last entry
	  CounterWeb.Endpoint
	]

	# See https://elixir.hexdocs.pm/Supervisor.html
	# for other strategies and supported options
	opts = [strategy: :one_for_one, name: Counter.Supervisor]
	Supervisor.start_link(children, opts)
  end

  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
	CounterWeb.Endpoint.config_change(changed, removed)
	:ok
  end
end
