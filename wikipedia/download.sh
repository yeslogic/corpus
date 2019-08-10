#! /usr/bin/env bash

base=`dirname $0`

#date=20181001
date=20190801

mkdir -p $base/$date

for i in bn hi ta or te gu pa ml kn si ; do
    ( cd $base/$date ; wget https://dumps.wikimedia.org/${i}wiki/$date/${i}wiki-$date-pages-articles-multistream.xml.bz2 )
done

mkdir -p $base/../words

for i in bn hi ta or te gu pa ml kn si ; do
    echo $i
    bzip2 -dc $base/$date/${i}wiki-$date-pages-articles-multistream.xml.bz2 | $base/../target/release/corpus $i html > $base/../words/$i.wiki.$date
done

