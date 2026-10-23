defmodule Bcit.Schools.Student do
  use Ecto.Schema
  import Ecto.Changeset

  schema "students" do
	field :sid, :string
	field :firstname, :string
	field :lastname, :string
	field :gpa, :float

	timestamps(type: :utc_datetime)
  end

  @doc false
  def changeset(student, attrs) do
	attrs = for {k, v} <- attrs, into: %{}, do: {k, String.trim(v)}
	student
	|> cast(attrs, [:sid, :firstname, :lastname, :gpa])
	|> validate_required([:sid, :firstname, :lastname, :gpa])
	|> validate_format(:sid, ~r/^A\d{8}$/, message: "must start with 'A' followed by 8 digits")
	|> validate_length(:firstname, max: 25, min: 1, message: "firstname must be between 1 and 25 characters long")
	|> validate_number(:gpa, greater_than_or_equal_to: 0, less_than_or_equal_to: 100, message: "must be between 0 and 100 inclusive")
	|> unique_constraint(:sid)
  end
end
