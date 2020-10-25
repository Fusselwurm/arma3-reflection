#include "..\script_component.hpp"

/*
	result: String - data sent back from extension. It is up to extension maker what it is.
	returnCode: Number - integer return from extension method. It is up to extension maker to define it.
	errorCode: Number - error code in case of command error (see description). 0 means no errors.
*/
params [
	["_method", "", [""]],
	["_params", [], [[]]]
];

private _response = ("reflection" callExtension [_method, _params]);
_response params [
	["_result", "", [""]],
	["_returnCode", -1, [0]],
	["_errorCode", -1, [0]]
];
if (_result == "" && _returnCode == -1) then {
		ERROR("no answer for reflection.so call. extension broken? :(");
};

[_result, _returnCode, _errorCode]
