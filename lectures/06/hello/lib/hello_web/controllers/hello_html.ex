defmodule HelloWeb.HelloHTML do
	use HelloWeb, :html

	def index(assigns) do
		~H"""
		Hello from hell
		"""
	end

	embed_templates "hello_html/*"
end
