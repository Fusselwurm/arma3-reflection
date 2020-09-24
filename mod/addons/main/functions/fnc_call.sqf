#include "..\script_component.hpp"

/**
 * return [error, value] with error == "" if everything is fine
 */
params [
	["_method", "", [""]],
	["_params", [], [[]]]
];

private _call = format ["%1(%2)", _method, _params joinString ","];

private _rawResponse = ("reflection" callExtension _call);
if (count _rawResponse == 0) exitWith {
		ERROR("no answer for reflection.so call :(");
		[-1]
};

(call compile _rawResponse) params [
		["_responseCode", 0, [0]],
		["_returnValue", "", [""]]
];

[_responseCode, _returnValue]