#include <string.h>     // strcmp, strncpy
#include <algorithm>
#include <sstream>
#include <boost/filesystem.hpp>
#include <boost/filesystem/fstream.hpp>
#include "getCommandLine.cpp"
#include <string>

namespace fs = boost::filesystem;
using namespace std;

static char version[] = "212e7c8";
static string cmdLineString = "";

string getParameterValue(string parameterString, string parName) {
	
	int parFileIndex = parameterString.find("-" + parName + "=");
	if (parFileIndex == -1) { // oh great, let's get the parameter file...
		return "";
	}
	
	string filename = parameterString.substr(parFileIndex + parName.length() + 2);
	int parFileEnd = filename.find(' ');
	if (parFileEnd != -1) {
		filename = filename.substr(0, parFileEnd);
	}
	return filename;
}

string getArgsFileContents() {
	string filename = getParameterValue(cmdLineString, "par");
	
	if (!fs::exists(filename)) {
		cerr << "warning: cannot find parameter file " << filename;
		return "";
	}
	
	fs::ifstream fIn;
	fIn.open(filename.c_str(), std::ios::in);

	if (!fIn) {
		cerr << "string load_data_in_str(string fname)" << endl;
		cerr << "Error reading the file: " << filename << endl;
		return "";
	}
	
	stringstream ss;
	ss << fIn.rdbuf();
	
	return ss.str();
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

void RVExtension(char *output, int outputSize, const char *function)
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

/*
int main( int argc, const char* argv[] )
{	
	cout << getGamePort();
	return 0;
}
*/