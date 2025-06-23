#!/bin/bash

set -xeu

# install llvm from choco in place of vcpkg to speed things up
choco install -y llvm
# vcpkg install llvm  # It's too slow
vcpkg install opencv4[contrib,nonfree]