rust-plantronics
================

This is just a little toy project that'll connect to plantronics hub
and pull the mute status. It currently writes to a [tasmota REST API](https://github.com/arendst/Sonoff-Tasmota) 
on an [Sonoff S26 IoT plug](https://www.itead.cc/sonoff-s26-wifi-smart-plug.html)
to mute or unmute an "on the air" sign like this one: 
[On Air Sign: Aliexpress](https://www.aliexpress.com/item/LB480-On-Air-Recording-Studio-NEW-NR-LED-Neon-Light-Sign-home-decor-crafts/1000006552370.html).

The end result is...

![Animated](https://i.imgur.com/msRk3HK.gif)

Note; plantronics hub REST api is kinda crusty. Structures are not
easily coerced into static types (ie: isError adds a field, but the 
result is either empty or missing, but not consistently).


```
rust-plantronics 0.0.1
Derek Anderson <derek@armyofevilrobots.com>
Monitors state of a plantronics headset and sends events to various endpoints.

USAGE:
    rust-plantronics [OPTIONS] --tasmota <tasmota>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>        Sets a custom config file
    -T, --tasmota <tasmota>    The destination url for the tasmota rest api (http://sonoff-on-air.local/)
    -u, --url <url>            The BaseURL of the plantronics API (http://localhost:32017/)
```
