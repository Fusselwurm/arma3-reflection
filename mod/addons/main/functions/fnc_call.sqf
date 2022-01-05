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

switch (_returnCode) do {
    case -1: {ERROR("no answer for reflection.so call. extension broken? :(");};
    case 0: {TRACE_1("successful call to %1", _method)};
    case 1: {ERROR_1("command %1 not found", _method)};
    case 4: {ERROR_1("attempted to write value larger than buffer in command %1", _method);};
    default {
        if (_returnCode >= 20 && _returnCode < 30) then {
            ERROR_2("method %1 got %2 arguments", _method, count _params);
        };
        if (_returnCode >= 30 && _returnCode < 40) then {
            ERROR_2("method %1 got invalid argument type at position %2", _method, _returnCode % 10);
        };
    };
};

[_result, _returnCode, _errorCode]
