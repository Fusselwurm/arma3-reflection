#include <string.h>     // strcmp, strncpy
#include <algorithm>
#include <sstream>
#include <sys/stat.h>
#include <fstream>
#include <string>
#include "getCommandLine.cpp"

using namespace std;

static char version[] = "46f0056";
static string cmdLineString = "";

inline bool file_exists (const string& name) {
	ifstream f(name.c_str());
	if (f.good()) {
		f.close();
		return true;
	} else {
		f.close();
		return false;
	}
}

string file_get_contents(string filename) {
	string str;
	ifstream t(filename);
	string str((istreambuf_iterator<char>(t)), istreambuf_iterator<char>());

	return str;
}

string getParameterValue(string parameterString, string parName) {

	int parameterPos = parameterString.find("-" + parName + "=");
	if (parameterPos == -1) {
		return "";
	}

	string filename = parameterString.substr(parameterPos + parName.length() + 2);
	int parFileEnd = filename.find(' ');
	if (parFileEnd != -1) {
		filename = filename.substr(0, parFileEnd);
	}
	return filename;
}

string getArgsFileContents() {
	string filename = getParameterValue(cmdLineString, "par");

	if (!file_exists(filename)) {
		cerr << "warning: cannot find parameter file " << filename;
		return "";
	}

	return file_get_contents(filename)
}


string getCompleteCommandLine() {
	cmdLineString = getCommandLine();

	string argsFileContents = getArgsFileContents();
	replace(argsFileContents.begin(), argsFileContents.end(), '\n', ' ');
	string completeParamString = cmdLineString + " " + argsFileContents;

	return completeParamString;
}

string getGamePort() {
	return getParameterValue(getCompleteCommandLine(), "port");
}

extern "C" void RVExtension(char *output, int outputSize, const char *function)
{
	cmdLineString = getCommandLine();

	if (!strcmp(function, "version"))
	{
		strncpy(output, version, outputSize);
	}
	else if (!strcmp(function, "gameport"))
	{
		strncpy(output, getGamePort().c_str(), outputSize);
	} else {
		strncpy(output, "unknown function", outputSize);
	}

	output[outputSize - 1] = '\0';

	return;
}
