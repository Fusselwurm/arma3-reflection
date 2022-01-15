#include "..\script_component.hpp"

if (!GVAR(logCmdline)) exitWith {};

ISNILS(GVAR(lineLength), 256);

private _cmd = [] call EFUNC(commandline,raw);

INFO("### complete command line start ###");
while {count _cmd > 0} do {
    private _line = _cmd select [0, GVAR(lineLength)];
    INFO(_line);
    _cmd = _cmd select [GVAR(lineLength)];
};
INFO("### complete command line end ###");
