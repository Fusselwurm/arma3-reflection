#!/bin/bash

head=`git reflog --decorate -1 --no-color`

# if possible, use current tag...
#version=`echo $head | grep -o -E 'tag: \w+' | sed -e 's/tag: //'`
version=`echo $head | sed -re 's/^.*tag: ([0-9a-z\.\-]+).*$/\1/'`

if [[ "$head" == "$version" ]]; then
	# ...if not, use commit hash
	#	version=`echo $head | grep --color=never -o -E '^[0-9a-f]+'`
	version=`echo $head | sed -re 's/^([0-9a-f]+).*$/\1/g'`
fi

echo "current version: $version"

sed -i -e "s/^static char version\[\] = .*$/static char version\[\] = \"$version\";/" main.cpp
