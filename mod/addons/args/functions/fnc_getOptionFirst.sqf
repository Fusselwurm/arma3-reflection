#include "..\script_component.hpp"

/**
 * return [error, value] with error == "" if everything is fine
 */
params [
	["_optName", "", [""]]
];

assert(_optName != "");

["get_option_first", [_optName]] call EFUNC(main,call);