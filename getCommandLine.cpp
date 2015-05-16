#include <stdio.h>
#include <unistd.h>
#include <climits>
#include <string>

using namespace std;

char * getCommandLine() {
	char * path = NULL;
	char * msg = NULL;
	char * cmdline = new char[_POSIX_ARG_MAX];
	FILE * file;
	size_t read = 0;
	size_t i = 0;

	path = new char[1024];

	sprintf(path, "/proc/%ld/cmdline", (long) getpid());
	file = fopen(path, "r");

	if (file == NULL) {
		msg = new char[1024];
		sprintf(msg, "Could not open file \"%s\"", path);
		perror(msg);
		goto ERROR_EXIT;
	}

	read = fread(cmdline, sizeof(char), _POSIX_ARG_MAX, file);
	i = 0;
	while(i++ < read) {
		if (cmdline[i] == 0) {
			cmdline[i] = ' ';
		}
	}
	cmdline[read-1] = (char) NULL;

	ERROR_EXIT:
	if (path)
		delete path;
	if (msg)
		delete msg;
	if (file)
		fclose(file);

	return cmdline;
}
