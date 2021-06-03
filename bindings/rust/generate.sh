#/usr/bin/env bash
# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0


echo "Recommended Compilers: gcc-9 or gcc-10; set with export CC="
echo "If your OpenSSL package didn't come with pkg-config files, you'll need to point to "
echo "your OpenSSL libraries with OPENSSL_INCLUDE_DIR and OPENSSL_LIB_DIR"

# cd into the script directory so it can be executed from anywhere
cd "$(dirname "${BASH_SOURCE[0]}")"
ARGS="$@"

if [[ -d s2n-tls-sys/lib ]]; then
  rm -rf s2n-tls-sys/lib
fi
mkdir -p s2n-tls-sys/lib

# we copy the C sources into the `lib` directory so they get published in the
# actual crate artifact.
cp -r \
  ../../api \
  ../../crypto \
  ../../error \
  ../../pq-crypto \
  ../../stuffer \
  ../../tls \
  ../../utils \
  s2n-tls-sys/lib/

# Grab specific files to see if bindgen will build it
mkdir -p s2n-tls-sys/lib/testlib s2n-tls-sys/lib/tests
cp ../../tests/testlib/s2n_testlib.h s2n-tls-sys/lib/testlib
cp ../../tests/testlib/s2n_stuffer_hex.c s2n-tls-sys/lib/testlib
cp ../../tests/s2n_test.h s2n-tls-sys/lib/tests
cp ../../tests/unit/s2n_drbg_test.* s2n-tls-sys/lib/tests


# generate the bindings modules from the copied sources
cd generate && cargo run -- ../s2n-tls-sys $ARGS && cd ..

cd s2n-tls-sys
cargo build

# make sure everything builds and passes sanity checks
cd s2n-tls-sys \
  && cargo test \
  && cargo test --release \
  && cargo test --features quic \
  && cd ..

