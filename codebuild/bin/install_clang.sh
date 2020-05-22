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

set -ex

usage() {
	echo "install_clang.sh download_dir install_dir os_name"
	exit 1
}

if [ "$#" -ne "3" ]; then
	usage
fi

CLANG_DOWNLOAD_DIR=$1
CLANG_INSTALL_DIR=$2
PLATFORM=$3

if [[ -d "$CLANG_DOWNLOAD_DIR" ]]; then
	rm -rf "$CLANG_DOWNLOAD_DIR"
fi
mkdir -p "$CLANG_DOWNLOAD_DIR"
cd "$CLANG_DOWNLOAD_DIR"

# The Certificate used by chromium.googlesource.com is not in the default CA
# list supported by git/curl on Ubuntu/AL2, but the certificate is in the
# ca-certificates.crt file in Ubuntu/AL2, so set this env variable so that it is
# picked up by git.
if [ "$PLATFORM" == "linux" ]; then
	if [[ -f "/etc/system-release" ]]; then
		#TODO: if we're going to co-mingle AL2, we need a global flag/function todo the following.
		grep -q 'Amazon Linux release 2' /etc/system-release
		if [ "$?" == 0 ]; then
			export SSL_CERT_FILE=/etc/ssl/certs/ca-bundle.crt
		else
			export SSL_CERT_FILE=/usr/lib/ssl/certs/ca-certificates.crt
		fi
	fi
fi

export GIT_CURL_VERBOSE=1
echo "Downloading Clang..."
git clone https://chromium.googlesource.com/chromium/src/tools/clang

echo "Updating Clang..."
python3 "$CLANG_DOWNLOAD_DIR"/clang/scripts/update.py

# "third_party" directory is created above $CLANG_DOWNLOAD_DIR after running
# update, move it into $CLANG_DOWNLOAD_DIR once update is complete.
mv ../third_party "$CLANG_DOWNLOAD_DIR"

echo "Installed Clang Version: "
"$CLANG_DOWNLOAD_DIR"/third_party/llvm-build/Release+Asserts/bin/clang --version

# Install matching LLVM if FUZZ_COVERAGE is enabled
if [[ "$FUZZ_COVERAGE" == "true" ]]; then
	LLVM_INSTALL_DIR="$CLANG_INSTALL_DIR"/../llvm
	mkdir -p "$LLVM_INSTALL_DIR"
	python3 "$CLANG_DOWNLOAD_DIR"/clang/scripts/update.py --package="coverage_tools" --output-dir="$LLVM_INSTALL_DIR"
	ln -sf $LLVM_INSTALL_DIR/bin/llvm-cov /usr/bin/llvm-cov
	ln -sf $LLVM_INSTALL_DIR/bin/llvm-profdata /usr/bin/llvm-profdata
fi

mkdir -p "$CLANG_INSTALL_DIR" && cp -rf "$CLANG_DOWNLOAD_DIR"/third_party/llvm-build/Release+Asserts/* "$CLANG_INSTALL_DIR"

