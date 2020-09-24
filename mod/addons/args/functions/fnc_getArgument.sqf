#include "..\script_component.hpp"

/**
 * return [error, value] with error == "" if everything is fine
 */
params [
	["_argName", "", [""]]
];

assert(_argName != "");

["arg", [_argName]] call EFUNC(main,call);