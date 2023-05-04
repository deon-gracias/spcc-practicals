FILE="symbol_table"
gcc `pkg-config --cflags glib-2.0` $FILE.c `pkg-config --libs glib-2.0` -o $FILE.o
./$FILE.o