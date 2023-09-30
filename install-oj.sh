# https://github.com/online-judge-tools/oj/blob/master/docs/INSTALL.ja.md
sudo apt install python3
python3 --version

sudo apt install -y python3-pip
pip3 --version

pip3 install online-judge-tools

if !(type "oj" > /dev/null 2>&1); then
    (echo; echo 'export PATH="$HOME/.local/bin:$PATH"') >> ~/.zshrc
    source ~/.zshrc
fi

oj --version