# [言語処理100本ノック](http://www.cl.ecei.tohoku.ac.jp/nlp100/)

## memo

### CaboCha ###

コンパイルが難しいのでdocker containerのものを使う

    docker run --rm -i ocadaruma/cabocha cabocha -f 1 < data/neko.txt > neko.txt.cabocha
