"""
Recreating the brute force program interface in Mr Robot
"""
import time
from itertools import product
from pathlib import Path


SCRIPTED_LINES = (
    "1 packets transmittd, 1 received, 0% packet loss",
    "elpscrk -list pswList.list-add; 1984; 8D;",
    "elpscrk -ip 222.12.154.102 -user mich05654"
    )

def print_script(script=SCRIPTED_LINES, delay=2):
    """
    Printing the scripted lines to screen
    """
    for line in script:
        print(line)
        time.sleep(delay)


def check_password(guess: str, answer: str) -> bool:
    """Compares the guessed password to actual password"""
    return guess == answer


def crack_passord(target):
    """
    Runs a dictionary attack against a known password
    """
    result = None
    count = 0
    path = Path("passwords.txt")
    with path.open() as word_file:
        word_list = list(map(str.strip, word_file.readlines()))

        total = (len(word_list) * len(word_list)) + len(word_list)

        for word in word_list:
            count += 1
            print(f"Checking {count} of {total}", end="\r", flush=True)
            if check_password(word, target):
                result = word
                break

        if result is None:
            for words in product(word_list, word_list):
                word = "".join(words)
                count += 1
                print(f"Checking {count} of {total}", end="\r", flush=True)
                if check_password(word, target):
                    result = word
                    break

    return result

def main(target):
    """Start point for the program"""

    print_script()

    start_time = time.time()
    result = crack_passord(target)

    end_time = time.time()
    time_elapsed = end_time - start_time

    print(f"\nTime elasped: {time_elapsed}", end="\n", flush=False)

    if result:
        print(f"Password: {result}", end="\n", flush=False)
    else:
        print("Password was not found", end="\n", flush=False)

if __name__ == "__main__":
    main(target="123456seven")
