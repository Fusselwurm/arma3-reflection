#include "..\script_component.hpp"

params [
	["_optName", "", [""]]
];

assert(_optName != "");

(["get_option_first", [_optName]] call EFUNC(main,call))#0;
