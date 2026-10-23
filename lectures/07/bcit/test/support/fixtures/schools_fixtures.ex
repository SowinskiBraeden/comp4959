defmodule Bcit.SchoolsFixtures do
  @moduledoc """
  This module defines test helpers for creating
  entities via the `Bcit.Schools` context.
  """

  @doc """
  Generate a unique student sid.
  """
  def unique_student_sid do
    "A" <> (System.unique_integer([:positive]) |> Integer.to_string() |> String.pad_leading(8, "0"))
  end

  @doc """
  Generate a student.
  """
  def student_fixture(attrs \\ %{}) do
    {:ok, student} =
      attrs
      |> Enum.into(%{
        firstname: "some firstname",
        gpa: 85.5,
        lastname: "some lastname",
        sid: unique_student_sid()
      })
      |> Bcit.Schools.create_student()

    student
  end
end
