#!bash
cargo build -q

cat << EOF > perft_stockfish.exp
   set timeout 10
   lassign \$argv pos depth
   spawn stockfish
   send "position \$pos\\ngo perft \$depth\\n"
   expect "Nodes searched"
   send "quit\\n"
   expect eof
EOF

cat << EOF > perft_crust.exp
   set timeout 100
   lassign \$argv pos depth
   spawn target/debug/crust
   send "position \$pos\\ngo perft \$depth\\n"
   send "quit\\n"
   expect eof
EOF


expect perft_stockfish.exp "$1" $2 | grep -P '\w\d\w\d.*: \d+' > debug/left
expect perft_crust.exp "$1" $2 | grep -P '[a-z]\d[a-z]\d.*: \d+' > debug/right

rm perft_stockfish.exp
rm perft_crust.exp

sort -o debug/left debug/left
sort -o debug/right debug/right

diff debug/left debug/right