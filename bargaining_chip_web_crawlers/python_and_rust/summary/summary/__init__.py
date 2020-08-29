from typing import List

from .summary import get_summary, get_summary_references
from .summary import get_summary_iterator, get_summary_iterator_parallel

__all__ = [
    "get_summary",
    "get_summary_references",
    "get_summary_iterator",
    "get_summary_iterator_parallel",
    "get_summary_py"
]


def start_buffer(num: int, buf: int) -> int:
    result = num - buf

    while result < 0:
        result += 1

    return result


def end_buffer(num: int, limit: int, buf: int) -> int:
    # Add 1 to make the buf num inclusive
    result = num + buf + 1

    while result > limit:
        result -= 1

    return result


def get_summary_py(contents: str, target: str, buffer_lines: int) -> List[str]:
    result = []
    lines = contents.split("\n")
    total_num_of_lines = len(contents)

    for index, line in enumerate(lines):
        if target in line:
            start = start_buffer(index, buffer_lines)
            end = end_buffer(index, total_num_of_lines, buffer_lines)

            text_section = lines[start: end]
            result.append("\n".join(text_section))

    return result
