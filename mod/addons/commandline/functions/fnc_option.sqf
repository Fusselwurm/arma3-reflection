#include "..\script_component.hpp"

params [
	["_optName", "", [""]]
];

parseSimpleArray ((["commandline:option", [_optName]] call EFUNC(main,call))#0)
