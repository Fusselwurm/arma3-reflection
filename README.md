![master](https://github.com/Fusselwurm/arma3-reflection/workflows/compile/badge.svg)
![version](https://img.shields.io/github/v/release/Fusselwurm/arma3-reflection)

# arma3-reflection

This is an Arma3 mod that provides access to the Arma3 command line parameters.

## Installation

Get it from [Steam WS](https://steamcommunity.com/sharedfiles/filedetails/?id=2237948948) , from the [releases page](https://github.com/Fusselwurm/arma3-reflection/releases) or build it yourself.

## Usage: the functions

for example `[] call arma3_reflection_args_fnc_port` , `[parameterName] call arma3_reflection_args_fnc_getArgument`

## Usage: the extension

Use in SQF context. Return value is a string containing an SQF array of the format `[<int: error code>,<string: return value>]`.

* Error code `0` means success

### arg

`"reflection" callExtension "arg(<parametername>)"`

Example:

```
_response = ("reflection" callExtension "arg(port)");
_port = "0";
if ((count _response > 0)) then {
        _port = (call compile _response) select 1;
} else {
        diag_log "could not get game server port: " + _response;
};

```

### version

`"reflection" callExtension "version()"`

# Warning

This is me while doing C++ . You have been warned

![I have no idea what I'm doing](http://i3.kym-cdn.com/photos/images/facebook/000/234/765/b7e.jpg)
