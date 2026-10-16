defmodule Lab4b do
  for line <- String.split(File.read!("atomic-weights.txt"), "\n", trim: true) do
    [_number, symbol, _name, weight] = String.split(line)
    def atomic_weight(unquote(symbol)), do: unquote(String.to_float(weight))
  end

  def atomic_weight(_symbol), do: -1
end
