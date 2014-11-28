#!/bin/bash

for f in xml/*xml; do
    python2 r_client.py "$f";
done;
