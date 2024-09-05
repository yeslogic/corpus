#! /bin/bash

for s in bn gu hi kn ml or pa si ta te my ; do
    echo $s syllables
    cat words/$s.* | target/release/syllables $s > data/syl.$s
    grep -v ^bad data/syl.$s > data/good.$s
    grep ^bad data/syl.$s > data/bad.$s
done
