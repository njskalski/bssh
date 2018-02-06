#!/usr/bin/env python3

import os, sys, getpass

openssh_portable_repo = "https://github.com/openssh/openssh-portable"

reset = False
if len(sys.argv) > 1 and sys.argv[1] == "reset":
    reset == True

dir_path = os.path.dirname(os.path.realpath(__file__))
assert dir_path == os.getcwd(), "please run this script from it's directory."

openssh_bin_path = os.path.join(dir_path, "openssh")
openssh_repo_path = os.path.join(dir_path, "openssh-portable")

if reset:
    os.system("rm -rf openssh-portable")

os.system("rm -rf openssh")

if os.path.exists(openssh_repo_path):
    os.chdir(openssh_repo_path)
    assert 0 == os.system("git pull")
    os.chdir(dir_path)
else:
    assert 0 == os.system("git clone " + openssh-portable-repo), "unable to checkout openssh-portable, necessary for compatibility testing."

os.chdir(openssh_repo_path)

assert 0 == os.system("autoconf")
assert 0 == os.system("aclocal")
assert 0 == os.system("autoheader")

assert 0 == os.system( \
    "./configure --prefix=" + openssh_bin_path + \
    " --with-audit=debug --with-pid-dir=" + openssh_bin_path + \
    " --with-privsep-path=" + os.path.join(openssh_bin_path, "var/empty") + \
    " --with-privsep-user=" + getpass.getuser())

assert 0 == os.system("make -j8")
assert 0 == os.system("make install")

os.chdir(dir_path)

assert 0 == os.system("cp compat_tests/etc0/* openssh/etc/")
