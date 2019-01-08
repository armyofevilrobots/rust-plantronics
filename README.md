rust-plantronics
================

This is just a little toy project that'll connect to plantronics hub
and pull the mute status.

Note; plantronics hub REST api is kinda crusty. Structures are not
easily coerced into static types (ie: isError adds a field, but the 
result is either empty or missing, but not consistently).


