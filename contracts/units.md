# Engineering units

| Quantity | Representation | Unit |
| --- | --- | --- |
| Feed and tank pressure | Finite floating point | kPa |
| Temperature | Finite floating point | K |
| Battery energy | Finite floating point | Wh |
| Bus demand and heater power | Finite floating point | W |
| Throttle and state of charge | Closed interval [0,1] | Fraction |
| Position | Three finite components | m |
| Velocity | Three finite components | m/s |
| Torque | Three finite components | N m |
| Rate | Finite floating point | rad/s |
| Clock and sample timestamps | Unsigned integer | ms |

The UI performs presentation formatting only. Conversion at a subsystem boundary
must be explicit. No controller accepts NaN or infinity as a missing-value marker.
Invalid measurements carry quality metadata and fail the relevant interlock.

Coast reserve schedules express pack energy in Wh and planning duration in
seconds. Convert the available energy to watt-seconds before dividing by the
horizon to report discretionary draw in W. A charge fraction is dimensionless;
it must not be substituted directly for an energy or power budget.
