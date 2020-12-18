#!/bin/bash
# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License").
# You may not use this file except in compliance with the License.
# A copy of the License is located at
#
#  http://aws.amazon.com/apache2.0
#
# or in the "license" file accompanying this file. This file is distributed
# on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
# express or implied. See the License for the specific language governing
# permissions and limitations under the License.
#

# Shim code to get local docker/ec2 instances bootstraped like a CodeBuild instance.
# Not actually used by CodeBuild.

set -eu
source ./codebuild/bin/s2n_setup_env.sh

dev9ppa() {
    echo "We need a test PPA for gcc-9, cmake,psmis on Ubuntu18"
    add-apt-repository ppa:ubuntu-toolchain-r/test -y
    add-apt-repository ppa:longsleep/golang-backports -y
    apt-get update -o Acquire::CompressionTypes::Order::=gz
    apt-get update -y
}

prlimit() {
    # If prlimit is not on our current PATH, download and compile prlimit manually. s2n needs prlimit to memlock pages
    if ! type prlimit > /dev/null && [[ ! -d "$PRLIMIT_INSTALL_DIR" ]]; then
        mkdir -p "$PRLIMIT_INSTALL_DIR";
        codebuild/bin/install_prlimit.sh "$(mktemp -d)" "$PRLIMIT_INSTALL_DIR";
    fi
    }

# Main
if [[ ${DISTRO} != "ubuntu" ]]; then
    echo "Target ubuntu; running on $DISTRO: Nothing to do."
    exit 0
fi

DEPENDENCIES="unzip make psmisc sudo indent iproute2 kwstyle net-tools libssl-dev tcpdump valgrind lcov m4 nettle-dev nettle-bin pkg-config gcc g++ zlibc zlib1g-dev python3-pip python3-testresources llvm curl git tox cmake libtool ninja-build golang-go quilt gcc g++"

if [[ -n "$GCC_VERSION" ]] && [[ "$GCC_VERSION" != "NONE" ]]; then
    DEPENDENCIES+=" gcc-$GCC_VERSION g++-$GCC_VERSION";
fi
if [[ $(LATEST_CLANG) != "true" ]]; then
    DEPENDENCIES+=" clang-3.9 llvm-3.9";
fi

dev9ppa
prlimit
apt-get -y install --no-install-recommends ${DEPENDENCIES}
