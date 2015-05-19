#include "getCommandLine.h"
#include <string.h>
#include <string>
#include <algorithm>
#include <sstream>
#include <sys/stat.h>
#include <streambuf>
#include <fstream>
#include <string>
#include <iostream>
#include <vector>

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
	string argsFileContents = getArgsFileContents();
	replace(argsFileContents.begin(), argsFileContents.end(), '\n', ' ');
	string completeParamString = args + " " + argsFileContents;

	return completeParamString;
}
string getGamePort() {
	return getParameterValue(getCompleteCommandLine(), "port");
}
std::string escapeString(const std::string inString)
{
	std::vector<char> retString;
	for (int i = 0; i < inString.length(); i++)
	{
		const char& c = inString[i];
		switch (c)
		{
		case '"': retString.push_back('"'); retString.push_back('"'); break;
		default: retString.push_back(c);
		}
	}
	retString.push_back('\0');
	return std::string(retString.data());
}
extern "C" void RVExtension(char *output, int outputSize, const char *cmd)
{
	//Set a global variable with current command line and make sure we just do it once
	if (args.empty())
		args = getCommandLine();
	//put incomming command into string object for simpler access
	const std::string cmdString = std::string(cmd);

	//get extract function and param out of cmdString
	int openIdx = cmdString.find('(');
	if (openIdx == -1 || cmdString.find_last_of(')') != cmdString.size() - 1)
	{
		strncpy(output, "[1,\"bad syntax\"]", outputSize);
		return;
	}
	const std::string fn = cmdString.substr(0, openIdx);
	const std::string param = cmdString.substr(openIdx + 1, cmdString.length() - openIdx - 2);

	//prepare size_t argument for later use
	size_t len;

	//resolve function names
	if (fn.compare("version") == 0)
	{

		std::string response("[0,\"");
		response = response.append(version).append("\"]");
		len = response.size();
		if (len >= outputSize)
		{
			strncpy(output, "[1, \"outputSize too small\"]", outputSize);
			len = outputSize - 1;
		}
		else
		{
			strncpy(output, response.c_str(), len);
		}
	}
	else if (fn.compare("arg") == 0)
	{
		std::string str = std::string("[0,\"").append(escapeString(getParameterValue(getCompleteCommandLine(), param))).append("\"]");
		len = str.length();
		if (len >= outputSize)
		{
			strncpy(output, "[1, \"outputSize too small\"]", outputSize);
			len = outputSize - 1;
		}
		else
		{
			strncpy(output, str.c_str(), len);
		}
	}
	else if (fn.compare("cmdLine") == 0)
	{
		std::string str = std::string("[0,\"").append(escapeString(getCompleteCommandLine())).append("\"]");
		len = str.length();
		if (len >= outputSize)
		{
			strncpy(output, "[1, \"outputSize too small\"]", outputSize);
			len = outputSize - 1;
		}
		else
		{
			strncpy(output, str.c_str(), len);
		}
	}
	else
	{
		char* str = "[1,\"unknown function. known functions: version(), arg(<paramName>)\"]";
		len = strlen(str);
		if (len >= outputSize)
		{
			strncpy(output, "[1, \"outputSize too small\"]", outputSize);
			len = outputSize - 1;
		}
		else
		{
			strncpy(output, str, len);
		}
	}

	//add final \0 to mark end of string
	output[len] = '\0';
}
#ifdef _DEBUG
#define TEST_OUTPUTSIZE 256
#define RUN_TEST(TESTNAME, ARG) memset(test_output, 0, TEST_OUTPUTSIZE);\
								cout << "Running Test: '" << TESTNAME << "' with '" << ARG << "'" << endl;\
								try {\
									RVExtension(test_output, TEST_OUTPUTSIZE, ARG);\
									cout << "\tTest Result: '" << test_output << "'" << endl;\
								}\
								catch(std::exception e) {cout << "\tTest Failed: " << e.what() << endl; }

int main( int argc, const char* argv[] ) {
	char test_output[TEST_OUTPUTSIZE];
	RUN_TEST("Argument", "arg(foo)");
	RUN_TEST("CommandLine", "cmdLine()");
	return 0;
}
#endif
