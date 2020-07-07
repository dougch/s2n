/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License").
 * You may not use this file except in compliance with the License.
 * A copy of the License is located at
 *
 *  http://aws.amazon.com/apache2.0
 *
 * or in the "license" file accompanying this file. This file is distributed
 * on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 * express or implied. See the License for the specific language governing
 * permissions and limitations under the License.
 *
 */

#include "crypto/s2n_fips.h"
#include "s2n_test.h"
#include "tests/testlib/s2n_testlib.h"
#include "tls/s2n_cipher_preferences.h"
#include "tls/s2n_cipher_suites.h"
#include "tls/s2n_kem.h"

#define RSP_FILE_NAME "kats/hybrid_ecdhe_bike_r1.kat"
#define SERVER_KEY_MESSAGE_LENGTH 2875
#define CLIENT_KEY_MESSAGE_LENGTH 2610

int main(int argc, char **argv)
{
    BEGIN_TEST();

#if !defined(S2N_NO_PQ)

    if (s2n_is_in_fips_mode()) {
        /* There is no support for PQ KEMs while in FIPS mode */
        END_TEST();
    }

    EXPECT_SUCCESS(s2n_test_hybrid_ecdhe_kem_with_kat(&s2n_bike1_l1_r1, &s2n_ecdhe_bike_rsa_with_aes_256_gcm_sha384,
                                                      "KMS-PQ-TLS-1-0-2019-06", RSP_FILE_NAME,
                                                      SERVER_KEY_MESSAGE_LENGTH, CLIENT_KEY_MESSAGE_LENGTH));

#endif

    END_TEST();
}
