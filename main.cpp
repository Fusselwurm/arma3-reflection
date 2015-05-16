#include <string.h>     // strcmp, strncpy
#include <algorithm>
#include <sstream>
#include <sys/stat.h>
#include <streambuf>
#include <fstream>
#include <string>
#include "getCommandLine.cpp"
#include <iostream>

using namespace std;

static char version[] = "46f0056";
static string args = "";

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

string file_get_contents(const string& filename) {
	ifstream inFile;
	inFile.open(filename.c_str());//open the input file

	stringstream strStream;
	strStream << inFile.rdbuf();//read the file
	return strStream.str();//str holds the content of the file
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
	string filename = getParameterValue(args, "par");

	if (!file_exists(filename)) {
		// "warning: cannot find parameter file " << filename;
		return "";
	}

	return file_get_contents(filename);
}


string getCompleteCommandLine() {
	args = getCommandLine();

	string argsFileContents = getArgsFileContents();
	replace(argsFileContents.begin(), argsFileContents.end(), '\n', ' ');
	string completeParamString = args + " " + argsFileContents;

	return completeParamString;
}

string getGamePort() {
	return getParameterValue(getCompleteCommandLine(), "port");
}

extern "C" void RVExtension(char *output, int outputSize, const char *cmd)
{
	args = getCommandLine();

	string cmdString = string(cmd);
	string result;
	int openIdx = cmdString.find('(');
	if (openIdx == -1) {
		strncpy(output, "1:bad syntax", outputSize);
		cout << openIdx;
		return;
	}
	if (cmdString.find_last_of(')') != cmdString.size() - 1) {
		strncpy(output, "1, bad syntax", outputSize);
		return;
	} 	
	string fn = cmdString.substr(0, openIdx);
	string param = cmdString.substr(openIdx + 1, cmdString.length() - openIdx - 2);

	if (!strcmp(fn.c_str(), "version"))
	{
		strncpy(output, version, outputSize);
	}
	else if (!strcmp(fn.c_str(), "arg"))
	{
		string val = getParameterValue(getCompleteCommandLine(), param);
		strncpy(output, (string("0,") + val).c_str(), outputSize);
	} else {
		strncpy(output, "1,unknown function. known functions: version(), arg(<paramName>)", outputSize);
	}

	output[outputSize - 1] = '\0';

	return;
}
/*
int main( int argc, const char* argv[] ) {
	char foo[100];
	RVExtension(foo, 100, "arg(foo)");
	cout << foo;
	return 0;
}
*/