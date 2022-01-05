#include "..\script_component.hpp"

params [
	["_optName", "", [""]]
];

parseSimpleArray ((["get_option", [_optName]] call EFUNC(main,call))#0)
