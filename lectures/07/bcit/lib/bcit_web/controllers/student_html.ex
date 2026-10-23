defmodule BcitWeb.StudentHTML do
  use BcitWeb, :html

  embed_templates "student_html/*"

  @doc """
  Renders a student form.

  The form is defined in the template at
  student_html/student_form.html.heex
  """
  attr :changeset, Ecto.Changeset, required: true
  attr :action, :string, required: true
  attr :return_to, :string, default: nil

  def student_form(assigns)
end
