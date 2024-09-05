#! /usr/bin/env bash

base=`dirname $0`

#date=20181001
date=20190801
my_date=20240601 # Myanmar

mkdir -p $base/$date $base/$my_date

function download() {
    local i=$1
    local date=$2

    ( cd $base/$date ; curl -OL https://dumps.wikimedia.org/${i}wiki/$date/${i}wiki-$date-pages-articles-multistream.xml.bz2 )
}

function build-corpus() {
    local i=$1
    local date=$2

    echo $i
    bzip2 -dc $base/$date/${i}wiki-$date-pages-articles-multistream.xml.bz2 | $base/../target/release/corpus $i html > $base/../words/$i.wiki.$date
}

for i in bn hi ta or te gu pa ml kn si my ; do
    if [ $i = "my" ]; then
        download $i $my_date
    else
        download $i $date
    fi
done

mkdir -p $base/../words

for i in bn hi ta or te gu pa ml kn si my ; do
    if [ $i = "my" ]; then
        build-corpus $i $my_date
    else
        build-corpus $i $date
    fi
done
