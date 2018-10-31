#! /bin/bash

for s in bn gu hi kn ml or pa si ta te ; do
    echo $s words
    ./corpus $s < wikipedia/${s}wiki-20181001-pages-articles-multistream.xml > data/words.$s
done

for s in bn gu hi kn ml or pa si ta te ; do
    echo $s syllables
    ./syllables $s < data/words.$s > data/syl.$s
    grep -v ^bad data/syl.$s > data/good.$s
    grep ^bad data/syl.$s > data/bad.$s
done
