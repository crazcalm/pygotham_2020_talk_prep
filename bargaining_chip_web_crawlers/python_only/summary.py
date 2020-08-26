import time
from pathlib import Path
from typing import List


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


def get_summary(text: str, target: str, buffer_lines=2) -> List[str]:
    result = []
    for index, line in enumerate(text):
        if target in line:
            start = start_buffer(index, buffer_lines)
            end = end_buffer(index, len(text), buffer_lines)

            text_section = text[start: end]
            result.append("\n".join(text_section))

    return result


def main(path_to_document: str, target: str, buffer_lines=2):
    start_time = time.time_ns()

    text = Path(path_to_document).read_text().split("\n")
    result = get_summary(text, target)

    for summary in result:
        print(f"{summary}\n\n")

    end_time = time.time_ns()
    time_elasped = end_time - start_time

    print(f"\n\nTime elasped: {time_elasped} in nano seconds")


if __name__ == "__main__":
    result = main("peter_pan.txt", "Peter")
