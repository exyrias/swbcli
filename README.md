# SwitchBot API CLI Tool

A simple CLI tool for interacting with the [SwitchBot API](https://github.com/OpenWonderLabs/SwitchBotAPI). This tool allows you to list devices, check device statuses, and send commands to devices.

## Installation

This project is written in Rust and can be installed using `cargo`.

Run the following commands to install the tool:

```sh
cargo install swbcli
```

## Usage

This CLI tool interacts with the SwitchBot API to perform various actions such as listing devices, checking device statuses, and sending commands.
Results will be returned in JSON format.

### Command Format

```sh
swbcli --token|-t <token> --secret|-s <secret> <action> [options]
```

- `--token` or `-t`: Your SwitchBot API token (required)
- `--secret` or `-s`: Your SwitchBot API secret (required)
- `<action>`: The action to perform (required)
  - `list`: List all devices
  - `status`: Get the status of a specific device
  - `control`: Send a command to a specific device

### Action-Specific Options

- `list`: No additional options
- `status`:
  - `--device_id` or `-i`: The ID of the device (required)
- `control`:
  - `--device_id` or `-i`: The ID of the device (required)
  - `--cmd` or `-c`: The command to send (required)
  - `--param` or `-p`: The command parameters in JSON format (required)

### Examples

#### List all devices

```sh
swbcli -t <TOKEN> -s <SECRET> list
```

#### Get the status of a device

```sh
swbcli -t <TOKEN> -s <SECRET> status -i <DEVICE_ID>
```

#### Send a command to a device

```sh
# Bot: Turn on the switch
swbcli -t <TOKEN> -s <SECRET> control -i <DEVICE_ID> -c turnOn -p default
```

For detailed information about commands and parameters, please refer to https://github.com/OpenWonderLabs/SwitchBotAPI.

## License

This project is licensed under the [MIT License](LICENSE).