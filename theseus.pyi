from pathlib import Path
from typing import Literal, TextIO

__all__ = ["Config", "Color", "ColorGenerator", "Label", "Report"]

NOT_GIVEN = NotImplemented

class Config:
    """
    Configuration options for a report.
    """

    def __init__(
        self,
        *,
        cross_gap: bool = False,
        compact: bool = False,
        underlines: bool = True,
        multiline_arrows: bool = True,
        color: bool = True,
        tab_width: int = 4,
        ascii: bool = False,
        byte_indexed: bool = False,
        label_attach: Literal["start", "middle", "end"] = "middle",
    ):
        """
        Args:
            cross_gap:
                When label lines cross one-another, should there be a gap?
                The alternative to this is to insert crossing characters.
                However, these interact poorly with label colours.
            compact:
                If true, minimize used space in the report.
            underlines:
                If false, hide underlines under the label spans.
            multiline_arrows:
                If true, show arrows pointing to the start and end of multilines
                spans.
            color:
                If false, disable colors in the report.
            tab_width:
                The number of spaces to use for a tab character.
            ascii:
                If true, use ASCII characters instead of Unicode ones for the
                report. This is useful for terminals that do not support
                Unicode.
            byte_indexed:
                If true, the report will use byte indices instead of utf-8
                codepoint indices. Byte indexing is faster, but is more
                incovenient to work with. In general, use byte_indexed=True if
                your source code is bytes or ascii and byte_indexed=False if
                it is a string.
            label_attach:
                Where inline labels should attach to their spans.
        """

class Color:
    """
    Store terminal colors.
    """

    def __init__(
        self,
        value: (
            int
            | Literal[
                "primary",
                "black",
                "white",
                "red",
                "green",
                "blue",
                "yellow",
                "cyan",
                "magenta",
                "bright-black",
                "bright-white",
                "bright-red",
                "bright-green",
                "bright-blue",
                "bright-yellow",
                "bright-cyan",
                "bright-magenta",
            ]
        ),
    ) -> None:
        """
        Initialize a Color instance.

        Args:
            value (str | int):
                The color value, either as a string or an integer. The integer
                represent a color code in the 0-255 range.
        """

    @staticmethod
    def rgb(red: int, green: int, blue: int) -> Color:
        """
        Create a Color instance from RGB values.

        Args:
            red:
                The red component (0-255).
            green:
                The green component (0-255).
            blue:
                The blue component (0-255).
        """

class ColorGenerator:
    """
    Generate unique colors.
    """

    def __init__(self) -> None:
        """
        Initialize a ColorGenerator instance.
        """

    def __iter__(self) -> ColorGenerator: ...
    def __next__(self) -> Color: ...
    def next(self) -> Color:
        """
        Get the next color in the sequence.
        """

class Label:
    """
    Represents a label pointing to some span of code.
    """

    def __init__(
        self,
        start: int,
        end: int,
        *,
        path: str | Path = NOT_GIVEN,
        message: str = NOT_GIVEN,
        color: Color = NOT_GIVEN,
        order: int = NOT_GIVEN,
        priority: int = NOT_GIVEN,
    ):
        """
        Args:
            start, end:
                The starting position of the label.
            path:
                The file path for the file pointed by the span. If not given,
                uses the default file for the repost.
            message:
                The message associated with the label.
            color:
                An optional color for the label.
            order:
                A number indicating the order of this label compared to the
                other labels in the report.
            priority:
                A number indicating the priority of highlighting this label
                compared to the other labels in the report.
        """

    def copy(
        self,
        *,
        message: str = NOT_GIVEN,
        color: Color = NOT_GIVEN,
        order: int = NOT_GIVEN,
        priority: int = NOT_GIVEN,
    ) -> Label:
        """
        Copy label possibly replacing some of the attributes.
        """

class Report:
    """
    A report about errors, warnings and advise for a source code file (or a
    collection of them)
    """

    def __init__(
        self,
        source: str | Path | TextIO,
        start: int,
        end: int,
        code: int = NOT_GIVEN,
        message: str = NOT_GIVEN,
        kind: str = "error",
        color: Color = NOT_GIVEN,
        labels: list[Label] = NOT_GIVEN,
        notes: list[str] = NOT_GIVEN,
        helps: list[str] = NOT_GIVEN,
        config: Config = NOT_GIVEN,
        files: list[Path | TextIO] | dict[Path | str, str] = NOT_GIVEN,
    ):
        """
        Args:

            source:
                The souce string. If a Path or file-like object is given, the
                source is read from the corresponding file. It is usually
                better to pass the later, since it will be used to print the
                name of the file in the report.
            start, end:
                The starting and ending positions of the report in the source
                code.
            code:
                The main error message for this report.
            message:
                A message describing the report.
            kind:
                The kind of the report, e.g., "error", "warning", "advice". If
                any other custom string is given, a color must be provided.
            color:
                An optional color for custom report kinds. Cannot be set for
                standard kinds like "error", "warning" or "advice".
            labels:
                A list of labels associated with this report.
            notes:
                A list of notes associated with this report.
            helps:
                A list of help messages associated with this report.
            config:
                Configuration options for the report.
            files:
                A list of files associated with this report or a dictionary mapping
                file paths to their contents.
        """

    def print(self, stderr: bool = False):
        """
        Print the report to the console.

        Args:
            stderr: If True, print to stderr instead of stdout.
        """

    def color(self) -> Color:
        """
        Generates a new unique random color.
        """

    def add_label(self, label: Label):
        """
        Add a label to the report.
        """

    def label(
        self,
        start: int,
        end: int,
        *,
        path: str | Path = NOT_GIVEN,
        message: str = NOT_GIVEN,
        color: Color = NOT_GIVEN,
        order: int = NOT_GIVEN,
        priority: int = NOT_GIVEN,
    ) -> Label:
        """
        Create a new label and add it to the report.

        See the `Label` class for more details on the parameters.
        """

    def add_note(self, note: str):
        """Add a note to the report."""

    def add_help(self, help: str):
        """Add a help message to the report."""
