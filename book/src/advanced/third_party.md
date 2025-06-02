# Third Party Libs

## Virtual Enviroment

NeoCurl tries to find and use a Python Interpreter from a `VIRTUAL_ENV` and load modules installed there.
I recommend using the `venv` directory next to the script.

To create a virtual enviroment run:

```bash
python3.11 -m venv venv
```

Make sure to use **python3.11**, as that is the version NeoCurl is compiled with.

Activate it:

```bash
source ./venv/bin/activate
```

## Install libraries

With a venv active, install libs:

```bash
pip3 install libs
```

## Run

Make sure venv is active and run the script:

```bash
ncurl run definition
```

Libraries should be loaded.

## Debug

Set `RUST_LOG=debug` to see debug messages. This might help debug venv.
