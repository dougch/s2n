#!/bin/bash
# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
#
# Licensed under the Apache License, Version 2.0 (the "License"). You may not use
# this file except in compliance with the License. A copy of the License is
# located at
#
# http://aws.amazon.com/apache2.0/
#
# or in the "license" file accompanying this file. This file is distributed on an
# "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
# implied. See the License for the specific language governing permissions and
# limitations under the License.

set -eu

BUILDSPEC_OMNIBUS=./spec/buildspec_omnibus.yml
BUILDSPEC_OMNIBUS_CODECOMMIT=./spec/buildspec_omnibus_codecommit.yml

BUILDSPEC_FUZZ=./spec/buildspec_fuzz_batch.yml
BUILDSPEC_INTEG=./spec/buildspec_integ_batch.yml
BUILDSPEC_GENERAL=./spec/buildspec_general_batch.yml
CODECOMMIT_PREFIX="third-party-src"

synth_subjobs () {
  yq -S -Y -r '{batch:{"build-list":[.batch."build-list"[]| select(.identifier|contains("Fuzz")) ]}}' $BUILDSPEC_OMNIBUS > $BUILDSPEC_FUZZ
  yq -S -Y -r '{batch:{"build-list":[.batch."build-list"[]| select(.identifier|contains("Integ")) ]}}' $BUILDSPEC_OMNIBUS > $BUILDSPEC_INTEG
  yq -S -Y -r '{batch:{"build-list":[.batch."build-list"[]| select(.identifier|contains("Fuzz")|not)|select(.identifier|contains("Integ")|not) ]}}' $BUILDSPEC_OMNIBUS > $BUILDSPEC_GENERAL
  echo -e "Created:\n$BUILDSPEC_FUZZ\n$BUILDSPEC_INTEG\n$BUILDSPEC_GENERAL\n"
}


check_buildspec () {
    OMNIBUS=$(yq -r '.batch."build-list"|length' $BUILDSPEC_OMNIBUS)
    INTEG=$(yq -r '.batch."build-list"|length' $BUILDSPEC_INTEG)
    FUZZ=$(yq -r '.batch."build-list"|length' $BUILDSPEC_FUZZ)
    GENERAL=$(yq -r '.batch."build-list"|length' $BUILDSPEC_GENERAL)
    echo -e "Checking the math on newly created buildspec files\n$OMNIBUS = $INTEG + $FUZZ + $GENERAL"
    if (($OMNIBUS != $INTEG+$FUZZ+$GENERAL)); then
      echo "Counts do not match!"
    fi
}

codecommit_pathfix(){
  sed 's/buildspec: /buildspec: '$CODECOMMIT_PREFIX'\//' "$BUILDSPEC_OMNIBUS" > "$BUILDSPEC_OMNIBUS_CODECOMMIT"
  echo -e "Created:\n $BUILDSPEC_OMNIBUS_CODECOMMIT"
}

PREREQS="jq yq"
for i in $PREREQS; do
  if ! command -v $i &> /dev/null; then
     echo "$i needs to be install (use pip)"
  fi;
done
synth_subjobs
check_buildspec
codecommit_pathfix
yamllint "$BUILDSPEC_OMNIBUS" "$BUILDSPEC_OMNIBUS_CODECOMMIT"
echo "Note the buildspec_*_batch.yml files that were just created should only be used in-line with CodeBuild and not be commited to the repository."