#include "..\script_component.hpp"

private _cmdline = "";
private _idx = 0;
while {
	(["commandline:raw", [_idx]] call EFUNC(main,call)) params [
		["_value", "", [""]],
		["_returnCode", -1, [-1]],
		["_errorCode", -1, [-1]],
	];

	_cmdline += _value;
	_idx += 1;

	(_value != "") && {_returnCode == 0} && {_errorCode == 0}
} do {

};

_cmdline
