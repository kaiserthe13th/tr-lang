#!/usr/bin/env python3
from shutil import which
from os import system as run, path
from sys import stderr
from colorama import init as colorama_init, Fore, Style

colorama_init(autoreset=True)

def does_prog_exist(prog):
    return which(prog) is not None

install_options = []

if does_prog_exist("wget"):
    install_options.append("wget")
    install_options.append("wget(prerelease)")
if does_prog_exist("cargo"):
    install_options.append("cargo(install)")
    if path.exists("Cargo.toml") and path.isdir("src"):
        install_options.append("cargo(install from local)")
    install_options.append("cargo(install from git[may be unstable])")
if does_prog_exist("curl") and not does_prog_exist("cargo"):
    install_options.append("install rust using curl & cargo(install)")

if len(install_options) > 1:
    print(f"{Fore.BLUE+Style.BRIGHT}Found {len(install_options)} methods of installation!")
    for i, j in enumerate(install_options): print(f'{Fore.YELLOW}{i+1}){Style.RESET_ALL} {j}')
    while True:
        method = input(f"{Style.BRIGHT}Choose your install method{Fore.GREEN}: ")
        try:
            method = int(method)
            if method > 0 and method < len(install_options):
                break
            else:
                print(f"{Fore.RED}Please enter a number from 1 to {len(install_options)}.{Style.RESET_ALL}", file=stderr)
        except ValueError:
            print(f"{Fore.RED}Please enter a whole number.{Style.RESET_ALL}", file=stderr)
elif install_options:
    print(f"The only install method found is {install_options[0]}")

