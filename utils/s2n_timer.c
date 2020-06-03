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

#include "utils/s2n_timer.h"

#include "tls/s2n_config.h"
#include "utils/s2n_result.h"
#include "utils/s2n_safety.h"

S2N_RESULT s2n_timer_start(struct s2n_config *config, struct s2n_timer *timer)
{
    GUARD_AS_RESULT(config->monotonic_clock(config->monotonic_clock_ctx, &timer->time));

    return S2N_RESULT_OK;
}

S2N_RESULT s2n_timer_elapsed(struct s2n_config *config, struct s2n_timer *timer, uint64_t *nanoseconds)
{
    uint64_t current_time;

    GUARD_AS_RESULT(config->monotonic_clock(config->monotonic_clock_ctx, &current_time));

    *nanoseconds = current_time - timer->time;

    return S2N_RESULT_OK;
}

S2N_RESULT s2n_timer_reset(struct s2n_config *config, struct s2n_timer *timer, uint64_t *nanoseconds)
{
    uint64_t previous_time = timer->time;

    GUARD_RESULT(s2n_timer_start(config, timer));

    *nanoseconds = timer->time - previous_time;

    return S2N_RESULT_OK;
}
