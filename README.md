![Linux build](https://github.com/Fusselwurm/arma3-reflection/workflows/C/Linux%20x86%20build/badge.svg)

# arma3-reflection

This is an arma3 server extension that provides access to the Arma3  command line parameters.

## Installation

Compile either as VS project (Windows) or using the `make` (Linux).

Put the `reflection.so` or `dll` into your arma dir


## Usage

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
