#include "..\script_component.hpp"

private _result = ["port"] call FUNC(option);

if ((count _result) > 0) then {
	parseNumber (_result#0);
} else {
	0
};
