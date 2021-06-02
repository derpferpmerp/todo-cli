# Info

This was created so that I could code in rust, and
also so that you could display the hex, URL encoded,
and value of the users inputs (command line) in a pretty
Json output.

The output will look like this:
```json
{
  "Arg NUM": {
    "Value": "User Input",
    "Hex": "Hex Value",
    "Url": "Url Encoded Value"
  }
}
```

# Building

Linux:
* Install Cargo (Linux): `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

Windows:
`Download the executable online and install VSC's C++ Tools`

Then you need to clone my repository, cd to it

Next:
`cargo run ARG1 ARG2 ...`
