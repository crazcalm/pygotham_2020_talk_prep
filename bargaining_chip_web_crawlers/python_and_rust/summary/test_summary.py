import unittest
from pathlib import Path

import pytest

from summary import get_summary, get_summary_references, get_summary_py
from summary import get_summary_iterator, get_summary_iterator_parallel

"""
class TestSummaryBenchmark(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        path = Path("peter_pan.txt")
        cls.text = path.read_text().split("\n")
        cls.target = "Peter"

    @benchmark
    def test_rust_get_summary(self, benchmark):
        result = benchmark(get_summary, self.text, self.target)
        self.assertEqual(len(result), 3000)

    @benchmark
    def test_rust_get_summary_py(self, benchmark):
        result = benchmark(get_summary_py, self.text, self.target)
        self.assertEqual(len(result), 3000)
"""

text = Path("peter_pan.txt").read_text()
target = "Peter"
buf = 2


def test_rust_get_summary(benchmark):
    result = benchmark(get_summary, text * 100, target, buf)
    assert len(result) == 40100


def test_rust_get_summary_iterator(benchmark):
    result = benchmark(get_summary_iterator, text * 100, target, buf)
    assert len(result) == 40100


def test_rust_get_summary_iterator_parallel(benchmark):
    result = benchmark(get_summary_iterator_parallel, text * 100, target, buf)
    assert len(result) == 40100


def test_rust_get_summary_refrences(benchmark):
    result = benchmark(get_summary_references, text * 100, target, buf)
    assert len(result) == 40100


def test_rust_get_summary_py(benchmark):
    result = benchmark(get_summary_py, text * 100, target, buf)
    assert len(result) == 40100
