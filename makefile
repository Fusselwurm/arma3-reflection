SHELL=/bin/bash

all: release

debug: CC = g++ -g
debug: reflection.so

release: CC = g++ -O3 
release: reflection.so

setup: ./insertVersion.sh

reflection.so: clean setup main.o getCommandLine.o
	@echo 'Building target: $@'
	@echo 'Invoking: Cross G++ Linker'
	${CC} -m32 -shared -o reflection.so  main.o getCommandLine.o
	@echo 'Finished building target: $@'

main.o: 
	@echo 'Building target: $@'
	@echo 'Invoking: Cross G++ Compiler'
	${CC} -Wall -c -fmessage-length=0 -m32 -fPIC -o main.o arma3-reflection/main.cpp
	@echo 'Finished building target: $@'

getCommandLine.o: 
	@echo 'Building target: $@'
	@echo 'Invoking: Cross G++ Compiler'
	${CC} -Wall -c -fmessage-length=0 -m32 -fPIC -o getCommandLine.o arma3-reflection/getCommandLine.cpp
	@echo 'Finished building target: $@'



clean: 
	rm -f *.{o,so}

