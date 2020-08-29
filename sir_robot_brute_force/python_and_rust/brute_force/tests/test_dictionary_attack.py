import pytest

from brute_force import dictionary_attack, dictionary_attack_py


DICT_PATH = "passwords.txt"
FOUND_TARGET = "123456seven"


@pytest.mark.fast
def test_rust_dictionary_attack(benchmark):
    result = benchmark(dictionary_attack, DICT_PATH, FOUND_TARGET)
    assert result == FOUND_TARGET


@pytest.mark.fast
def test_rust_dictionary_attack_py(benchmark):
    result = benchmark(dictionary_attack_py, DICT_PATH, FOUND_TARGET)
    assert result == FOUND_TARGET
