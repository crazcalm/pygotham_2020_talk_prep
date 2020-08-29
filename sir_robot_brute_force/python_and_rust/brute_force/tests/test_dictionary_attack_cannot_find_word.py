import pytest

from brute_force import dictionary_attack, dictionary_attack_py


DICT_PATH = "passwords.txt"
CANNOT_FIND_TARGET = "123456Seven"


@pytest.mark.slow
def test_rust_dictionaty_attack_on_cannot_find_word(benchmark):
    result = benchmark(dictionary_attack, DICT_PATH, CANNOT_FIND_TARGET)
    assert result is None


@pytest.mark.slow
def test_rust_dictionaty_attack_py_on_cannot_find_word(benchmark):
    result = benchmark(dictionary_attack_py, DICT_PATH, CANNOT_FIND_TARGET)
    assert result is None
