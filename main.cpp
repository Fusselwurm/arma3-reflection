/*
* example.c
*
*   gcc -shared -fPIC -o example.so example.c
*/
#include <string.h>     // strcmp, strncpy
#include "getCommandLine.cpp"

static char version[] = "212e7c8";

void RVExtension(char *output, int outputSize, const char *function)
{

	char * cmdLine = getCommandLine();
	
	if (!strcmp(function, "version"))
	{
		strncpy(output, version, outputSize);
	}
	else
	{
		strncpy(output, function, outputSize);
	}
	output[outputSize-1]='\0';

	return;
}
