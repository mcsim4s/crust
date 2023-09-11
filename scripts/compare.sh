#!bash

sort -o debug/left debug/left
sort -o debug/right debug/right

diff debug/left debug/right