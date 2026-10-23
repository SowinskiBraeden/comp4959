defmodule CounterWeb.CounterLive do
  use CounterWeb, :live_view
  alias Phoenix.PubSub
  @pubsub Counter.PubSub

  def mount(_params, _session, socket) do
	if connected?(socket), do: PubSub.subscribe(@pubsub, "counter")
	counter = Counter.Worker.value()
	{:ok, assign(socket, counter: counter)}
  end

  def render(assigns) do
	~H"""
	<section style="position: fixed; left: 50%; top: 50%; transform: translate(-50%, -50%);">
		<h1>{@counter}</h1>
		<div>
			<.button phx-click="inc">INCREMENT</.button>
			<.button phx-click="dec">DECREMENT</.button>
		</div>
	</section>
	"""
  end

	def handle_event("dec", _payload, socket) do
		Counter.Worker.dec()
		PubSub.broadcast(@pubsub, "counter", :value_changed)
		{:noreply, socket}
	end

	def handle_event("inc", _payload, socket) do
		Counter.Worker.inc()
		PubSub.broadcast(@pubsub, "counter", :value_changed)
		{:noreply, socket}
	end

	def handle_info(:value_changed, socket) do
		counter = Counter.Worker.value()
		{:noreply, assign(socket, counter: counter)}
	end
end
