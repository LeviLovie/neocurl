# Installation

NeoCurl uses [Python](https://www.python.org/) version **3.11**!

## Install **python3.11**

```bash
brew install python3.11
```

## Create a venv for **python3.11**

```
python3.11 -m venv venv
```

## Specify `PYTHON_SYS_EXECUTABLE`

```bash
export PYTHON_SYS_EXECUTABLE=$(which python3.11)
```

## Install NeoCurl

```bash
cargo install neocurl@2.0.0-alpha.2
```

## Check

```bash
ncurl --version
```

```bash
neocurl 2.0.0-alpha.2
```
