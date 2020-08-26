import os

import pytest


from brute_force import dictionary_attack, dictionary_attack_py


DICT_PATH = "passwords.txt"
FOUND_TARGET = "123456seven"
CANNOT_FIND_TARGET = "123456Seven"

def test_rust_dictionary_attack(benchmark):
    result = benchmark(dictionary_attack, DICT_PATH, FOUND_TARGET)
    assert result == FOUND_TARGET

def test_rust_dictionary_attack_py(benchmark):
    result = benchmark(dictionary_attack_py, DICT_PATH, FOUND_TARGET)
    assert result == FOUND_TARGET

def test_rust_dictionaty_attack_on_cannot_find_word(benchmark):
    result = benchmark(dictionary_attack, DICT_PATH, CANNOT_FIND_TARGET)
    assert result is None

def test_rust_dictionaty_attack_py_on_cannot_find_word(benchmark):
    result = benchmark(dictionary_attack_py, DICT_PATH, CANNOT_FIND_TARGET)
    assert result is None
