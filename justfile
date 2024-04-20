@_default:
	just --list

# builds any .teeny file, defaulting to the included example
@build file="fib.teeny":
	cargo run {{file}}

# compiles the built c code
@_compile file: (build file)
	clang out.c -o teeny.out

# runs the compiled c file
@run file="fib.teeny": (_compile file)
	./teeny.out

# cleans up after itself
@clean:
	rm *.out out.c
