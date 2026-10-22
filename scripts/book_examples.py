# /// script
# requires-python = ">=3.9"
# dependencies = ["bit_field"]
# ///
"""Make the example pages of the book from the files in docs/examples.

For each example, deku-display-cli makes the reStructuredText page. This script changes
that page to Markdown for mdbook, and renders each bitfield diagram to an SVG file with
the renderer that sphinxcontrib-bitfield uses.

Usage: uv run scripts/book_examples.py <examples dir> <output dir>
"""

import json
import os
import re
import subprocess
import sys
from pathlib import Path

from bit_field import jsonml_stringify, render

# The page title of each example. An example that is not here gets its file name.
TITLES = {
    "ipv4": "IPv4 Header",
    "80211": "802.11 Frame Control",
    "enums": "Enums",
    "example": "Mixed Fields",
}

# The order of the examples in the book
ORDER = ["ipv4", "80211", "example", "enums"]

HEADING_LEVELS = {"=": 1, "-": 2, "~": 3}
ANCHOR = re.compile(r"^\.\. _(?P<name>[^:]+):$")
OPTION = re.compile(r"^:(?P<key>\w+):\s*(?P<value>.*)$")
ROLE_REF = re.compile(r":ref:`(?P<name>[^`]+)`")
LITERAL = re.compile(r"``(?P<text>[^`]+)``")


def cli_rst(example: Path) -> str:
    """Return the reStructuredText page that deku-display-cli makes for the example."""
    command = [
        "cargo", "run", "--quiet", "--locked", "-p", "deku-display-cli", "--",
        str(example), "--format", "rst",
    ]
    return subprocess.run(command, check=True, capture_output=True, text=True).stdout


def bitfield_options(lines: list[str]) -> dict:
    """Change the options of a bitfield directive to the arguments of bit_field.render."""
    options = {}
    for line in lines:
        match = OPTION.match(line)
        if match is None:
            raise ValueError(f"not a directive option: {line!r}")
        key, value = match["key"], match["value"]
        options[key] = int(value) if value else True
    return options


def indented_block(lines: list[str], start: int) -> tuple[list[str], int]:
    """Return the indented lines that start at `start`, without the indent.

    Blank lines in the block stay. The block stops at the first line with no indent.
    """
    block = []
    index = start
    while index < len(lines) and (lines[index].startswith("    ") or not lines[index]):
        block.append(lines[index][4:])
        index += 1
    while block and not block[-1]:
        block.pop()
    return block, index


def list_table(block: list[str]) -> list[str]:
    """Change a two-column list-table body to a Markdown table."""
    rows = []
    for line in block:
        stripped = line.strip()
        if stripped.startswith("* - "):
            rows.append([stripped[len("* - "):]])
        elif stripped.startswith("- "):
            rows[-1].append(stripped[len("- "):])
    header, *body = rows
    table = ["| " + " | ".join(header) + " |", "|" + "---|" * len(header)]
    table += ["| " + " | ".join(LITERAL.sub(r"`\g<text>`", cell) for cell in row) + " |" for row in body]
    return table


def inline(line: str) -> str:
    """Change the inline reStructuredText markup of a line to Markdown."""
    line = ROLE_REF.sub(r"[\g<name>](#\g<name>)", line)
    return LITERAL.sub(r"`\g<text>`", line)


def to_markdown(rst: str, title: str, source: str, images: Path) -> str:
    """Change the CLI page to Markdown, and write each diagram to an SVG file in `images`."""
    lines = rst.splitlines()
    out = [f"# {title}", ""]
    anchor = None
    diagram_count = 0
    index = 0
    while index < len(lines):
        line = lines[index]
        following = lines[index + 1] if index + 1 < len(lines) else ""

        if line == ".. contents::":
            _, index = indented_block(lines, index + 1)
            continue

        if match := ANCHOR.match(line):
            anchor = match["name"]
            index += 1
            continue

        underline = set(following)
        if line and len(underline) == 1 and underline <= HEADING_LEVELS.keys() and len(following) == len(line):
            level = HEADING_LEVELS[following[0]]
            # The CLI page title is replaced with the example title
            if level > 1:
                if anchor is not None:
                    out.append(f'<a id="{anchor}"></a>')
                    anchor = None
                out.append(f"{'#' * level} {line}")
            index += 2
            continue

        if line == ".. bitfield::":
            block, index = indented_block(lines, index + 1)
            split = block.index("")
            options = bitfield_options(block[:split])
            fields = json.loads("\n".join(block[split + 1:]))
            diagram_count += 1
            name = f"{diagram_count}.svg"
            (images / name).write_text(jsonml_stringify(render(fields, **options)))
            out.append(f"![bitfield diagram]({images.name}/{name})")
            continue

        if line.startswith(".. list-table::"):
            block, index = indented_block(lines, index + 1)
            split = block.index("")
            out += list_table(block[split + 1:]) + [""]
            continue

        out.append(inline(line))
        index += 1

    out += ["", "## Source", "", "```rust,noplayground", f"{{{{#include {source}}}}}", "```"]
    return re.sub(r"\n{3,}", "\n\n", "\n".join(out)).strip() + "\n"


def main() -> None:
    if len(sys.argv) != 3:
        sys.exit("usage: book_examples.py <examples dir> <output dir>")
    examples_dir, out_dir = Path(sys.argv[1]), Path(sys.argv[2])
    out_dir.mkdir(parents=True, exist_ok=True)

    examples = sorted(examples_dir.glob("*.rs"), key=lambda path: (
        ORDER.index(path.stem) if path.stem in ORDER else len(ORDER), path.stem))
    for example in examples:
        images = out_dir / example.stem
        images.mkdir(exist_ok=True)
        for old in images.glob("*.svg"):
            old.unlink()
        # mdbook resolves an include path from the directory of the page
        source = Path(os.path.relpath(example.resolve(), out_dir.resolve())).as_posix()
        title = TITLES.get(example.stem, example.stem)
        page = to_markdown(cli_rst(example), title, source, images)
        (out_dir / f"{example.stem}.md").write_text(page)
        print(out_dir / f"{example.stem}.md")


if __name__ == "__main__":
    main()
