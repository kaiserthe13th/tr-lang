#!/usr/bin/env python3
import tempfile
import time
from subprocess import STDOUT, check_output
from sys import stderr, argv as args
from os.path import join as join_paths
import os
from typing import Callable, List, Optional, Union
from colorama import init as colorinit, Fore, Style

COLOR=False if '--color' in args or '-c' in args else None
NO_COLOR='--no-color' in args or '-n' in args

colorinit(strip=NO_COLOR or COLOR)
TMPD = tempfile.TemporaryDirectory(prefix="tr-lang.test").name
print(f"{Fore.BLUE+Style.BRIGHT}writing to {Style.RESET_ALL+Fore.YELLOW}{TMPD}")

beg = time.time_ns()
failed = []

DCR_SYM = '&' if os.name == 'nt' else ';'


def test(
    test_name: str,
    expected: Union[str, List[str], Callable],
    input: Optional[str] = None,
):
    print(f"{Fore.BLUE+Style.BRIGHT}running{Style.RESET_ALL}", test_name, "...", end=" ")
    t0 = time.time_ns()
    rf = str(
        check_output(
            f"{join_paths('.', 'target', 'debug', 'tr-lang')} y {join_paths('tests', f'{test_name}.trl')} {DCR_SYM} exit 0",
            shell=True,
            stderr=STDOUT,
            input=bytes(input, encoding="utf8") if input else input,
        ),
        encoding="utf8",
    )
    T = time.time_ns() - t0
    print(f"{Fore.BLUE}took{Style.RESET_ALL}", T // 1000000, "miliseconds", end=" :: ")
    if isinstance(expected, str) and rf == expected:
        print(f"{Fore.GREEN+Style.BRIGHT}success{Style.RESET_ALL}")
    elif isinstance(expected, list) and rf in expected:
        print(f"{Fore.GREEN+Style.BRIGHT}success{Style.RESET_ALL}")
    elif isinstance(expected, str):
        print(f"{Fore.RED+Style.BRIGHT}failure{Style.RESET_ALL}", file=stderr)
        print("found:", rf, sep="\n")
        print("expected:", expected, sep="\n")
        failed.append(test_name)
    elif isinstance(expected, list):
        print(f"{Fore.RED+Style.BRIGHT}failure{Style.RESET_ALL}", file=stderr)
        print("found:", rf, sep="\n")
        print("expected one of:")
        for i, j in enumerate(expected):
            print("Expectation", i + 1)
            print(j)
        failed.append(test_name)
    else:
        if expected(rf):
            print(f"{Fore.GREEN+Style.BRIGHT}success{Style.RESET_ALL}")
        else:
            print(f"{Fore.RED+Style.BRIGHT}failure{Style.RESET_ALL}", file=stderr)
            print("found:", rf, sep="\n")
            print("found didn't pass through", expected)


test("hello-world", expected="Hello, World!\n")
test("merhaba-dünya", expected="Merhaba, Dünya!\n")
test(
    "variables",
    expected=[
        "\n[WARNING] tests/variables.trl, Line 0, Column 0\n"
        "    StackNotEmpty: stack is not empty, if you aren't sure about why, you might want to take a look at you code\n"
        '    variables left in the stack(5) [1.1, "A long\\nstring", doğru]\n',
        
        "\n[WARNING] tests/variables.trl, Line 0, Column 0\n"
        "    StackNotEmpty: stack is not empty, if you aren't sure about why, you might want to take a look at you code\n"
        '    variables left in the stack(5) [1.1, "A long\\r\\nstring", doğru]\n',
    ],
)
test(
    "değişkenler",
    expected=[
        "\n[WARNING] tests/değişkenler.trl, Line 0, Column 0\n"
        "    StackNotEmpty: stack is not empty, if you aren't sure about why, you might want to take a look at you code\n"
        '    variables left in the stack(5) [1.1, "Uzun bir\\nyazı", doğru]\n',
        
        "\n[WARNING] tests/değişkenler.trl, Line 0, Column 0\n"
        "    StackNotEmpty: stack is not empty, if you aren't sure about why, you might want to take a look at you code\n"
        '    variables left in the stack(5) [1.1, "Uzun bir\\r\\nyazı", doğru]\n',
    ],
)
test("scope", expected="31")
test("order", expected="11")
test("multiples_of_5_or_3", expected="33\n" "234168\n")
for i in ("factorial", "faktoriyel"):
    test(
        i,
        expected="1! = 1\n"
        "2! = 2\n"
        "3! = 6\n"
        "4! = 24\n"
        "5! = 120\n"
        "100! = 93326215443944175354307254139643190247129328132295862491935879110669343325734178368282822618707234467717279847537548956702435362278960753539491860335688679424\n",
    )
for i in ("looping", "tekrarlama"):
    test(i, expected="10\n9\n8\n7\n6\n5\n4\n3\n2\n1\n")
test("if-else", expected="true\n")
test("ise-yoksa", expected="doğru\n")
test(
    "block",
    expected=['3\n4\n{"a": 3, "b": 4}\n7\n', '3\n4\n{"b": 4, "a": 3}\n7\n'],
)
test("type-conv", expected="Enter a number: 124.321\ndoğru\n", input="1")
test(
    "mini-calc",
    expected="Enter first number: Enter operation(+-*/): Enter second number: 31\n",
    input="10\n+\n21",
)

print(
    f"script {Fore.BLUE+Style.BRIGHT}took{Style.RESET_ALL}",
    (time.time_ns() - beg) // 1000000,
    "miliseconds total",
)
if not failed:
    print(f"{Fore.GREEN+Style.BRIGHT}all tests passed{Style.RESET_ALL}")
else:
    print(
        f"{Fore.BLUE}{len(failed)}{Style.RESET_ALL} tests {Fore.RED+Style.BRIGHT}failed{Style.RESET_ALL}"
    )
    for t in failed:
        print(f"{t} {Fore.RED}failed")
    print(f"{Fore.BLUE+Style.BRIGHT}removing {Style.RESET_ALL+Fore.YELLOW}{TMPD}")
    exit(1)

print(f"{Fore.BLUE+Style.BRIGHT}removing {Style.RESET_ALL+Fore.YELLOW}{TMPD}")
