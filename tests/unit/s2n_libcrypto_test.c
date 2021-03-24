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
 */

#include <s2n.h>
#include <stdlib.h>
#include "s2n_test.h"

#include "crypto/s2n_fips.h"

#include "tls/s2n_config.h"
#include "tls/s2n_connection.h"
#include "tls/s2n_security_policies.h"
#include "tls/s2n_tls13.h"

/* 
Example responses from s2n_get_openssL_version:
0x1010109f Openssl 1.1.1i 
0x1010107f Openssl 1.1.1g
*/
#define OPENSSL_MAJ_MIN_VERSION 0x10101000 /* Openssl 1.1.1x */

int main(int argc, char **argv)
{
    BEGIN_TEST();    

    /* Test: s2n_libcrypto_test check version */
    {
        long int openssl_version = s2n_get_openssl_version();
        /* Zero out the patch version to compare
        printf("OpenSSL version:  0x%lx\n", openssl_version); */
        EXPECT_EQUAL(OPENSSL_MAJ_MIN_VERSION, openssl_version>>8<<8);
    }

    END_TEST();
}
