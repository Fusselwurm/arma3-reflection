SHELL=/bin/bash

all: release

debug: CC = g++ -g
debug: reflection.so

release: CC = g++ -O3 
release: reflection.so

reflection.so: clean main.o
	@echo 'Building target: $@'
	@echo 'Invoking: Cross G++ Linker'
	${CC} -m32 -shared -o reflection.so  main.o
	@echo 'Finished building target: $@'

main.o:
	@echo 'Building target: $@'
	@echo 'Invoking: Cross G++ Compiler'
	${CC} -Wall -c -fmessage-length=0 -m32 -fPIC -o main.o main.cpp
	@echo 'Finished building target: $@'

clean: 
	rm -f *.{o,so}

