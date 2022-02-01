#!/bin/bash

set GH_DEB_INST "tr-lang_0.3.1_amd64.deb"
set GH_DEB_HREF "https://github.com/kaiserthe13th/tr-lang/releases/download/tr-lang-0.3.1/$GH_DEB_HREF"

echo "This script is outdated!"
echo "Please use install.py instead!"
echo "If you still want to..."
read -p "Do you still wish to install tr-lang on your system using this script[y/N]? " yn
case $yn in
    [Yy]* ) ;;
    * ) echo "exiting..."; exit;;
esac

set RETURN_DIR $PWD

cd ~
mkdir -p ".tr-lang"
cd .tr-lang
touch "config.yml"
echo "lang: auto
" > config.yml

cd ~
if [ -x "$(command -v cargo)" ]; then
    echo "using cargo to install..."
    echo "running: cargo install tr-lang"
    cargo install tr-lang
elif [ -x "$(command -v brew)" ]; then
    echo "using brew to install..."
    echo "running: brew tap kaiserthe13th/tr-lang"
    brew tap kaiserthe13th/tr-lang
    echo "running: brew install tr-lang"
    brew install tr-lang
elif [ -x "$(command -v dpkg)" -a -x "$(command -v wget)"]; then
    echo "using wget and dpkg to install..."
    echo "running: wget \"$GH_DEB_HREF\""
    wget $GH_DEB_HREF
    echo "running: sudo dpkg -i \"$GH_DEB_INST\""
    sudo dpkg -i "$GH_DEB_HREF"
elif [ -x "$(command -v curl)" ]; then
    echo "No Installer found on your system."
    echo "To continue installation, tr-lang install script wants to install rust"
    read -p "Do you wish to install rust on your system[y/N]? " yn
    case $yn in
        [Yy]* ) echo "installing rust installer...";;
        * ) echo "exiting..."; exit;;
    esac
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    echo "running: cargo install tr-lang"
    cargo install tr-lang
else
    echo "Didn't find any way to install tr-lang."
    echo "You must manually install (rust | brew | (dpkg & wget) | curl) for this script to work"
fi

cd $RETURN_DIR