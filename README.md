# arma3-reflection
sqf scripts get access to server CL parameters


This is an arma3 server extension that provides access to the arma3  command line parameters.

## Installation

Put the `reflection.so` into your arma dir

## Usage

Use in SQF context. Return value is a string with format `<error code>,<return value>`.

* Error code `0` means success

### arg

`"reflection" callExtension "arg(<parametername>)"`

### version

`"reflection" callExtension "version()"`

# Warning

This is me while doing C++ . You have been warned

![I have no idea what I'm doing](http://i3.kym-cdn.com/photos/images/facebook/000/234/765/b7e.jpg)
