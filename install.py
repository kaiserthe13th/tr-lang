#!/usr/bin/env python3
from shutil import which
from os import system as os_system, path, geteuid
from sys import stderr
from colorama import init as colorama_init, Fore, Style
import re

# Use default format method to format it
TRL_GH_RELEASES_B_STR = "https://github.com/kaiserthe13th/tr-lang/releases/download/tr-lang-{version}/{file}"
TRL_GH_RELEASES_LATEST_V = "0.3.1"
TRL_GH_RELEASES_LATEST_PRE_V = "0.4.0rc1"

# File sources for tr-lang gh releases
TRL_GH_RELEASES_LATEST_PRE_RPM = "tr-lang-0.4.0-0.rc1.x86_64.rpm"
TRL_GH_RELEASES_LATEST_PRE_DEB = "tr-lang_0.4.0.rc1_amd64.deb"
TRL_GH_RELEASES_LATEST_DEB = "tr-lang_0.3.1_amd64.deb"
TRL_GH_RELEASES_LATEST_RPM = None

def is_root() -> bool:
    return not geteuid()

def ex_input(prompt: str = "") -> str:
    try:
        g = input(str(prompt)+f"{Fore.GREEN}: ") if prompt else input(f"{Fore.GREEN}> ")
    except EOFError:
        print()
        exit()
    except KeyboardInterrupt:
        print()
        exit()
    print(end="")
    return g

LINK_REGEX = r"\b((?:https?://)?(?:(?:www\.)?(?:[\da-z\.-]+)\.(?:[a-z]{2,6})|(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)|(?:(?:[0-9a-fA-F]{1,4}:){7,7}[0-9a-fA-F]{1,4}|(?:[0-9a-fA-F]{1,4}:){1,7}:|(?:[0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}|(?:[0-9a-fA-F]{1,4}:){1,5}(?::[0-9a-fA-F]{1,4}){1,2}|(?:[0-9a-fA-F]{1,4}:){1,4}(?::[0-9a-fA-F]{1,4}){1,3}|(?:[0-9a-fA-F]{1,4}:){1,3}(?::[0-9a-fA-F]{1,4}){1,4}|(?:[0-9a-fA-F]{1,4}:){1,2}(?::[0-9a-fA-F]{1,4}){1,5}|[0-9a-fA-F]{1,4}:(?:(?::[0-9a-fA-F]{1,4}){1,6})|:(?:(?::[0-9a-fA-F]{1,4}){1,7}|:)|fe80:(?::[0-9a-fA-F]{0,4}){0,4}%[0-9a-zA-Z]{1,}|::(?:ffff(?::0{1,4}){0,1}:){0,1}(?:(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9])|(?:[0-9a-fA-F]{1,4}:){1,4}:(?:(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9])\.){3,3}(?:25[0-5]|(?:2[0-4]|1{0,1}[0-9]){0,1}[0-9])))(?::[0-9]{1,4}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])?(?:/[\w\.-]*)*/?)\b"
def highlight_links(s: str) -> str:
    r = ""
    lidx = 0
    matches = re.finditer(LINK_REGEX, s, re.MULTILINE)
    for m in matches:
        r += s[lidx:m.start()]
        r += f'{Fore.YELLOW}{s[m.start():m.end()]}{Fore.RESET}'
        lidx = m.end()
    r += s[lidx:]
    return r

def run(code: str) -> bool:
    styled_code = highlight_links(code)
    print(f"{Fore.BLUE+Style.BRIGHT}.---{'-'*len(code)}-.")
    print(f"{Fore.BLUE+Style.BRIGHT}|=>{Style.RESET_ALL} {styled_code} {Fore.BLUE+Style.BRIGHT}|")
    print(f"{Fore.BLUE+Style.BRIGHT}'---{'-'*len(code)}-'")
    return not os_system(code)

def choice_prompt_numbered(choices: list, prompt: str = "") -> str:
    for i, choice in enumerate(choices):
        print(f"{Fore.YELLOW}{i+1}){Style.RESET_ALL} {choice}")
    while True:
        g = ex_input(prompt)
        try:
            g = int(g)
            if g > 0 and g <= len(choices):
                g = choices[g-1]
                break
            else:
                print(f"{Fore.RED}Please enter a number from 1 to {len(choices)}.{Style.RESET_ALL}", file=stderr)
        except ValueError:
            print(f"{Fore.RED}Please enter a whole number.{Style.RESET_ALL}", file=stderr)
    print(end="")
    return g

colorama_init(autoreset=True)

def does_prog_exist(prog):
    return which(prog) is not None

if not is_root():
    print(f"""{Fore.RED}Script requires you to be root. Please rerun as root:{Style.BRIGHT+Fore.GREEN}
    ${Fore.BLUE} sudo su{Fore.YELLOW} -m   \
{Fore.RESET+Style.DIM}# -m preserves environment{Style.RESET_ALL+Style.BRIGHT+Fore.GREEN}
    #{Fore.BLUE} ./install.py{Style.RESET_ALL}""", file=stderr)
    exit(1)

install_options = []

if does_prog_exist("wget"):
    install_options.append("wget")
    install_options.append("wget(prerelease)")
if does_prog_exist("cargo"):
    install_options.append("cargo(install)")
    if path.exists("Cargo.toml") and path.isdir("src"):
        install_options.append("cargo(local)")
    install_options.append("cargo(git[may be unstable|broken])")
if does_prog_exist("curl") and not does_prog_exist("cargo"):
    install_options.append("install rust with curl then cargo(install)")

if install_options:
    print(f"""{Fore.YELLOW+Style.BRIGHT}<!>{Style.RESET_ALL+Fore.GREEN} \
You can exit using {Style.RESET_ALL}\
{Style.BRIGHT}Ctrl+D{Style.RESET_ALL+Fore.GREEN} or {Fore.RESET+Style.BRIGHT}Ctrl+C""")
    print(f"{Fore.BLUE+Style.BRIGHT}Found {len(install_options)} methods of installation!")
    method = choice_prompt_numbered(install_options, "Choose your installation method")

    print(f"Chose {method}")
    if method == "wget":
        installers = []
        if does_prog_exist("dpkg"):
            installers.append("dpkg")
        ### disabled because no rpm method for 0.3.1 ###
        # if does_prog_exist("rpm"):
        #     installers.append("rpm")
        
        if installers: installer = choice_prompt_numbered(installers, "Choose your installer")
        else:
            print("No installer found on system (dpkg, rpm)", file=stderr)
            exit(1)

        if installer == "dpkg":
            run(
                f"wget {TRL_GH_RELEASES_B_STR.format(version=TRL_GH_RELEASES_LATEST_V, file=TRL_GH_RELEASES_LATEST_DEB)}"
            ) and run(
                f"dpkg -i {TRL_GH_RELEASES_LATEST_DEB}"
            )
        ### disabled because no rpm method for 0.3.1 ###
        # elif installer == "rpm":
        #     run(f"wget {TRL_GH_RELEASES_B_STR.format(version=TRL_GH_RELEASES_LATEST_V, file=TRL_GH_RELEASES_LATEST_RPM)}")
        #     if does_prog_exist("tr-lang"):
        #         run(f"rpm -U {TRL_GH_RELEASES_LATEST_RPM}")
        #     else:
        #         run(f"rpm -i {TRL_GH_RELEASES_LATEST_RPM}")
    elif method == "wget(prerelease)":
        installers = []
        if does_prog_exist("dpkg"):
            installers.append("dpkg")
        if does_prog_exist("rpm"):
            installers.append("rpm")
        
        if installers: installer = choice_prompt_numbered(installers, "Choose your installer")
        else:
            print("No installer found on system (dpkg, rpm)", file=stderr)
            exit(1)

        if installer == "dpkg":
            run(f"wget {TRL_GH_RELEASES_B_STR.format(version=TRL_GH_RELEASES_LATEST_PRE_V, file=TRL_GH_RELEASES_LATEST_PRE_DEB)}")
            run(f"dpkg -i {TRL_GH_RELEASES_LATEST_DEB}")
        elif installer == "rpm":
            run(f"wget {TRL_GH_RELEASES_B_STR.format(version=TRL_GH_RELEASES_LATEST_PRE_V, file=TRL_GH_RELEASES_LATEST_PRE_RPM)}")
            if does_prog_exist("tr-lang"):
                run(f"rpm -U {TRL_GH_RELEASES_LATEST_PRE_RPM}")
            else:
                run(f"rpm -i {TRL_GH_RELEASES_LATEST_PRE_RPM}")
    elif method == "cargo(install)":
        run("cargo install tr-lang")
    elif method == "cargo(local)":
        run("cargo install tr-lang --path .")
    elif method == "cargo(git[may be unstable|broken])":
        run("cargo install tr-lang --git https://github.com/kaiserthe13th/tr-lang.git")
    elif method == "install rust with curl then cargo(install)":
        run("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh")
else:
    print(f"{Fore.RED}No installation method found.{Style.RESET_ALL}", file=stderr)
    exit(1)
