python3 -m venv .env
source .env/bin/activate

ln -rs scripts/make .env/bin/make

# sudo apt-get install cmake
# sudo apt-get install llvm lld lldb
# sudo apt-get install qemu-system-arm

#cmd=.env/bin/repo
#curl https://storage.googleapis.com/git-repo-downloads/repo -o $cmd
#chmod +x $cmd

make -h
