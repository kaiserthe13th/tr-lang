#!/usr/bin/env python3
import time
from os import mkdir
from subprocess import STDOUT, check_output, PIPE
from sys import stderr
from shutil import rmtree
from colorama import init as colorinit, Fore, Style

colorinit(autoreset=True)
TMPD = "/tmp/trl-tests"
mkdir(TMPD)

beg = time.time_ns()
failed = []

def test(test_name: str, expected: str, inp: str = None):
    print(f"{Fore.BLUE+Style.BRIGHT}running", test_name, "...", end=" ")
    t0 = time.time_ns()
    rf = str(check_output(
        f"target/debug/tr-lang y tests/{test_name}.trl; exit 0",
        shell=True,
        stderr=STDOUT,
        input=bytes(inp, encoding="utf8") if inp else inp 
    ), encoding="utf8")
    T = time.time_ns() - t0
    print(f"{Fore.BLUE}took", T//1000000, "miliseconds", end=" :: ")
    if rf == expected:
        print(f"{Fore.GREEN+Style.BRIGHT}success")
    else:
        print(f"{Fore.RED+Style.BRIGHT}failure{Style.RESET_ALL}", file=stderr)
        print("found:", rf, sep="\n")
        print("expected:", expected, sep="\n")
        failed.append(test_name)

test(
    "hello-world",
    "Hello, World!\n"
)
test(
    "merhaba-dünya",
    "Merhaba, Dünya!\n"
)
test(
    "variables",
    "[WARNING] tests/variables.trl, Line 0, Column 0\n"
    "    StackNotEmpty: stack is not empty, if you aren't sure about why, you might want to take a look at you code\n"
    '    variables left in the stack(5) [1.1, "A long\\nstring", doğru]\n'
)
test(
    "değişkenler",
    "[WARNING] tests/değişkenler.trl, Line 0, Column 0\n"
    "    StackNotEmpty: stack is not empty, if you aren't sure about why, you might want to take a look at you code\n"
    '    variables left in the stack(5) [1.1, "Uzun bir\\nyazı", doğru]\n'
)
test(
    "scope",
    "31"
)
test(
    "order",
    "11"
)
test(
    "multiples_of_5_or_3",
    "33\n"
    "234168\n"
)
for i in ("factorial", "faktoriyel"):
    test(
        i,
        "1! = 1\n"
        "2! = 2\n"
        "3! = 6\n"
        "4! = 24\n"
        "5! = 120\n"
        "100! = 93326215443944175354307254139643190247129328132295862491935879110669343325734178368282822618707234467717279847537548956702435362278960753539491860335688679424\n"
    )
for i in ("looping", "tekrarlama"):
    test(
        i,
        "10\n9\n8\n7\n6\n5\n4\n3\n2\n1\n"
    )
test(
    "if-else",
    "true\n"
)
test(
    "ise-yoksa",
    "doğru\n"
)
test(
    "type-conv",
    "Enter a number: 124.321\ndoğru\n",
    "1"
)
test(
    "mini-calc",
    "Enter first number: Enter operation(+-*/): Enter second number: 31\n",
    "10\n+\n21"
)

print(f"script {Fore.BLUE+Style.BRIGHT}took", (time.time_ns() - beg)//1000000, "miliseconds total")
if not failed:
    print(f"{Fore.GREEN+Style.BRIGHT}all tests passed")
else:
    print(f"{Fore.BLUE}{len(failed)}{Style.RESET_ALL} tests {Fore.RED+Style.BRIGHT}failed")
    for t in failed:
        print(f"{t} {Fore.RED}failed")

rmtree(TMPD)

