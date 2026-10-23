defmodule Bcit.SchoolsTest do
  use Bcit.DataCase

  alias Bcit.Schools

  describe "students" do
    alias Bcit.Schools.Student

    import Bcit.SchoolsFixtures

    @invalid_attrs %{sid: nil, firstname: nil, lastname: nil, gpa: nil}

    test "list_students/0 returns all students" do
      student = student_fixture()
      assert Schools.list_students() == [student]
    end

    test "get_student!/1 returns the student with given id" do
      student = student_fixture()
      assert Schools.get_student!(student.id) == student
    end

    test "create_student/1 with valid data creates a student" do
      valid_attrs = %{
        sid: "A00000001",
        firstname: "some firstname",
        lastname: "some lastname",
        gpa: 85.5
      }

      assert {:ok, %Student{} = student} = Schools.create_student(valid_attrs)
      assert student.sid == "A00000001"
      assert student.firstname == "some firstname"
      assert student.lastname == "some lastname"
      assert student.gpa == 85.5
    end

    test "create_student/1 with invalid data returns error changeset" do
      assert {:error, %Ecto.Changeset{}} = Schools.create_student(@invalid_attrs)
    end

    test "update_student/2 with valid data updates the student" do
      student = student_fixture()

      update_attrs = %{
        sid: "A00000002",
        firstname: "some updated firstname",
        lastname: "some updated lastname",
        gpa: 92.3
      }

      assert {:ok, %Student{} = student} = Schools.update_student(student, update_attrs)
      assert student.sid == "A00000002"
      assert student.firstname == "some updated firstname"
      assert student.lastname == "some updated lastname"
      assert student.gpa == 92.3
    end

    test "update_student/2 with invalid data returns error changeset" do
      student = student_fixture()
      assert {:error, %Ecto.Changeset{}} = Schools.update_student(student, @invalid_attrs)
      assert student == Schools.get_student!(student.id)
    end

    test "delete_student/1 deletes the student" do
      student = student_fixture()
      assert {:ok, %Student{}} = Schools.delete_student(student)
      assert_raise Ecto.NoResultsError, fn -> Schools.get_student!(student.id) end
    end

    test "change_student/1 returns a student changeset" do
      student = student_fixture()
      assert %Ecto.Changeset{} = Schools.change_student(student)
    end
  end
end
