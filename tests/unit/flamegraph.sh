#!/bin/bash
set -ex
flame_graph(){
    #args: testname directory/testname
    if [[ ! -d flamegraphs ]]; then
      mkdir flamegraphs
    fi
    flamegraph -o ./flamegraphs/$1.svg $2
}

#from the test/unit directory - so the test pem files can be found.
for i in $(find ../../build/bin -name s2n_\*_test -printf "%f\n"); do
  flame_graph $i ../../build/bin/$i
done
