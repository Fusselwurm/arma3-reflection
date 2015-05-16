/*
 * example.c
 *
 *   gcc -shared -fPIC -o example.so example.c
 */
#include <string.h>     // strcmp, strncpy
 
static char version[] = "1.0";
 
void RVExtension(char *output, int outputSize, const char *function)
{
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
