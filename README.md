# Neos Session Scanner

Checks [Neos VR](https://neos.com/) sessions to see if they're up.

## Usage

```
USAGE:
    neos-session-scanner.exe <SESSION_ID>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <SESSION_ID>     sets the session ID to check
```

## Details

Neos session have a session ID. It usually looks like this: `S-28a421ae-c1a3-429f-9496-3efb0e21187a`. You can use the Neos APi to check if sessions are up: `https://api.neos.com/api/sessions/S-28a421ae-c1a3-429f-9496-3efb0e21187a`. neos-session-scanner goes a level beyond that and actually attempts to open a connection to the session to see if it's there. This is useful in a few cases:
1. The session crashed before it could tell the Neos cloud it was closing. The Neos cloud thinks the session is up, but it isn't.
2. The session isn't listed in the Neos cloud, but it is actually up.

## Todo
- Clean up the output
- Run Neos API query and direct session query in parallel?
