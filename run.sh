#! /bin/bash

# takes about twenty minutes

for s in bn gu hi kn ml or pa si ta te ; do
    echo $s
    ./corpus $s < ${s}wiki-20181001-pages-articles-multistream.xml > out.$s
done
