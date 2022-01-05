private _settingsGroup = ["Arma3 Reflection"];

[
    QGVAR(logCmdline),
    "CHECKBOX",
    "Log Arma3 commandline",
    _settingsGroup,
    false,
    false,
    FUNC(logCmdline),
    false
] call CBA_fnc_addSetting;
