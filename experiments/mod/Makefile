
clean:
	rm and.s percent.s

nonopt: and.c percent.c
	gcc -S and.c percent.c
	diff and.s percent.s ||:

opt: and.c percent.c
	gcc -O -S and.c percent.c
	diff and.s percent.s ||:
