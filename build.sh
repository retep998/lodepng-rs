gcc src/lodepng.c -c -Ofast -o ${OUT_DIR}/lodepng.o
ar crus ${OUT_DIR}/liblodepng.a ${OUT_DIR}/lodepng.o