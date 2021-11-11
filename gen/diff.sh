#! /bin/bash

if [ -z "$1" ]
then
    echo -e "Error: a module name ust be supplied as argument"
    exit 1
fi

code --diff gen/previous/$1.rs gen/current/$1.rs
