#include "..\script_component.hpp"

private _result = ["port"] call FUNC(getOptionFirst);

if ((_result#2) != 0) exitWith {
		ERROR_1("could not get game server port from extension. return value '%1'", _result);
		0
};

parseNumber (_result#0);
