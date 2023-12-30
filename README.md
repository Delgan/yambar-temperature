# yambar-temperature

*A temperature module for [Yambar](https://codeberg.org/dnkl/yambar) status panel.*


## Installation

First build the software:

```bash
cargo build --release
```

Then install it:

```bash
sudo cp target/release/yambar-temperature /usr/bin/
```

## Usage

The `yambar-temperature` program will output sensor temperature at regular interval.

It produces the following tags that can be used by Yambar:

| Name        | Type  | Description                             |
| ----------- | ----- | --------------------------------------- |
| temperature | float | Average temperature of detected sensors |


## Configuration

The `yambar-temperature` command accepts three optional arguments:

| Option                       | Type            | Description                                                                                      |
| ---------------------------- | --------------- | ------------------------------------------------------------------------------------------------ |
| `--unit <unit>`              | string          | The unit of displayed temperature, one of `"celsius"` (default), `"fahrenheit"` or `"kelvin"`.   |
| `--poll-interval <interval>` | int             | The interval (in milliseconds) between each update. By default, `1000` is used.                  |
| `--names <names>...`         | list of strings | Names of sensors included in temperature calculation. By default, all sensors detected are used. |


See also `yambar-temperature --help`.

## Example

Here is a possible `config.yaml` for Yambar:

```yaml
bar:
  height: 32
  location: bottom
  background: 111111cc

  left:
    - script:
        path: /usr/bin/yambar-temperature
        args: [--poll-interval, 2000, --names, coretemp-isa-0000]
        content:
          - string:
              text: "[Temp] {temperature} °C"
```
