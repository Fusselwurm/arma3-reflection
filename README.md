![master](https://github.com/Fusselwurm/arma3-reflection/workflows/compile/badge.svg)
![version](https://img.shields.io/github/v/release/Fusselwurm/arma3-reflection)

# arma3-reflection

This is an Arma3 mod that provides access to the Arma3 command line parameters.

## Installation

Get it from [Steam WS](https://steamcommunity.com/sharedfiles/filedetails/?id=2237948948) , from the [releases page](https://github.com/Fusselwurm/arma3-reflection/releases) or build it yourself.

## Usage: the functions

### arma3_reflection_args module

This module contains functions to access the command line parameters Arma3 was started with.

#### arma3_reflection_args_fnc_getArgument

        [_paramaterName] call arma3_reflection_args_fnc_getArgument

#### arma3_reflection_args_fnc_port

        [] call arma3_reflection_args_fnc_port

## Usage: the extension

Use in SQF context. Return value is a string containing an SQF array of the format `[<int: error code>,<string: return value>]`.

* Error code `0` means success


### version

`"reflection" callExtension "version()"`
