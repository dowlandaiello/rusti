# rusti

A simple go-style utility for downloading, installing, and running rust programs.

## Installation

```zsh
cargo install rusti
```

## Usage

### Download a Package

```zsh
rusti get package_url
```

Example:

```zsh
rusti get github.com/dowlandaiello/rusti
```

### Install a Package

```zsh
rusti install package_url
```

Example:

```zsh
rusti install github.com/dowlandaiello/rusti
```

### Run the current Package

```zsh
rusti run
```

### Build the Current Package

```zsh
rusti build
```

### Create a new Package

```zsh
rusti init package_url
```

Example:

```zsh
rusti init github.com/dowlandaiello/rusti
```
