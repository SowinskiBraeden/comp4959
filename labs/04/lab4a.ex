defmodule Lab4a do
  for line <- String.split(File.read!("atomic-weights.txt"), "\n", trim: true) do
    [_number, symbol, _name, weight] = String.split(line)
    def unquote(:"#{String.downcase(symbol)}")(), do: unquote(String.to_float(weight))
  end
end
