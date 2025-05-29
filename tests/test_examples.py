import io
from contextlib import redirect_stdout
from itertools import islice
from pathlib import Path

from theseus import Color, ColorGenerator, Config, Label, Report  # type: ignore


class TestColor:
    def test_basic_color_functionality(self):
        red = Color("red")
        green = Color("green")
        one = Color(1)

        assert red != green
        assert red == Color("red")
        assert one == Color(1)
        assert str(red) == "Color('red')"

    def test_hash(self):
        red = Color("red")
        green = Color("green")
        one = Color(1)
        assert hash(red) != hash(green) != hash(one)

    def test_rgb(self):
        rgb = Color.rgb(0, 0, 0)
        assert rgb == Color.rgb(0, 0, 0)
        assert rgb != Color.rgb(1, 0, 0)
        assert str(rgb) == "Color.rgb(0, 0, 0)"

    def test_can_create_colors(self):
        assert str(Color("red")) == "Color('red')"
        assert str(Color(1)) == "Color(1)"
        assert str(Color.rgb(0, 0, 0)) == "Color.rgb(0, 0, 0)"


class TestColorGenerator:
    def test_color_generator(self):
        colors = ColorGenerator()
        assert isinstance(colors.next(), Color)

    def test_color_generator_is_iterable(self):
        colors = []
        for color in islice(ColorGenerator(), 10):
            assert isinstance(color, Color)
            colors.append(color)
        assert len(set(colors)) == 10


class TestLabel:
    def test_create_label(self):
        label1 = Label(0, 10, message="Bad code", color=Color("red"))
        label2 = Label(0, 10).copy(color=Color("red"), message="Bad code")
        label3 = label2.copy(message="Other message")
        assert label1 == label2
        assert label1 != label3
        assert label2 != label3

    def test_label_is_hashable(self):
        label1 = Label(0, 10, path="foo.py")
        label2 = Label(0, 10, path="bar.py")
        assert hash(label1) != hash(label2)

    def test_repr_label(self):
        label = Label(0, 10, message="Test")
        assert repr(label) == 'Label(0, 10, message="Test")'
        assert repr(label) == str(label)


class TestReport:
    def test_create_report_from_string(self):
        report = Report(
            "// code\nprint 'Hello, World!'/n... // more code",
            0,
            5,
            message="Bad function",
            config=Config(),
        )
        report.label(8, 13, message="Invalid command")
        report.label(13, 14, message="Print requires parenthesis")
        report.add_note("This is a note about the report.")
        report.add_note("You can learn more about this issue at https://google.com")
        report.add_help("Some help text here.")
        report.add_help("More help text")
        report.print()

    def test_create_report_from_io(self):
        fd = io.StringIO("print 'Hello, World!'\n")
        fd.name = "test.py"
        report = Report(fd, 0, 5, message="Bad function", config=Config(color=False))

        report.label(0, 5, message="Invalid command")
        with redirect_stdout(io.StringIO()) as data:
            report.print()

        output = data.getvalue()
        print(output)
        assert (
            output
            == """Error: Bad function
   ╭─[ test.py:1:1 ]
   │
 1 │ print 'Hello, World!'
   │     │  
   │     ╰── Invalid command
───╯
"""
        )

    def test_create_from_path(self):
        path = Path(__file__).parent / "example.lox"
        report = Report(
            path,
            start=5,
            end=21,
            message="Print command does not require parenthesis",
            kind="warning",
            config=Config(color=False),
        )
        report.label(
            start=5,
            end=6,
            message="Parenthesis start here",
            color=(color := report.color()),
        )
        report.label(
            start=20,
            end=21,
            message="And end here",
            color=color,
        )

        with redirect_stdout(io.StringIO()) as data:
            report.print()

        output = data.getvalue()
        print(output)
        assert (
            output
            == """Warning: Print command does not require parenthesis
   ╭─[ /home/chips/git/compiler/theseus/tests/example.lox:1:6 ]
   │
 1 │ print("Hello World!");
   │      │              │  
   │      ╰──────────────┼── Parenthesis start here
   │                     │  
   │                     ╰── And end here
───╯
"""
        )

    def test_compact(self):
        path = Path(__file__).parent / "example.lox"
        report = Report(
            path,
            start=5,
            end=21,
            message="Print command does not require parenthesis",
            kind="warning",
            config=Config(color=False, compact=True),
        )
        report.label(
            5,
            6,
            message="Parenthesis start here",
            color=(color := report.color()),
        )
        report.label(
            20,
            21,
            message="And end here",
            color=color,
        )

        with redirect_stdout(io.StringIO()) as data:
            report.print()

        output = data.getvalue()
        print(output)
        assert (
            output
            == """Warning: Print command does not require parenthesis
   ╭─[ /home/chips/git/compiler/theseus/tests/example.lox:1:6 ]
 1 │print("Hello World!");
   │     ╰──────────────┼─ Parenthesis start here
   │                    ╰─ And end here
"""
        )
