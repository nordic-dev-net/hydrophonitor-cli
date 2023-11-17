# Hydrophonitor CLI

Introduces a command line utility called `hp-cli` which interfaces with a Hydrophonitor, a Raspberry Pi based hydrophone passive acoustic monitoring system.

```
Usage: hp-cli <COMMAND>

Commands:
  import          Import data from a hydrophonitor device.
  clean           Clean a hydrophonitor device.
  update          Update a hydrophonitor device.
  flash           Flash a new hydrophonitor device or overwrite an old
                  installation.
  info            Get information about the device.
  debug           Different debugging options for the device.
  help            Print this message or the help of the given subcommand(s).

Options:
  -h, --help      Print help.
  --version       Print version.
```

Example:

```
hp-cli import ~/brygga-1 ~/Deployments --clean-imported
[SUCCESS] Import deployment 2023-11-09T10_20_01.041+0200 from brygga-1 to /home/kaskelotti/Deployments/2023-11-09T10_20_01.041+0200
[SUCCESS] Import deployment 2023-11-09T11_20_00.097+0200 from brygga-1 to /home/kaskelotti/Deployments/2023-11-09T11_20_00.097+0200
[SUCCESS] Clean device brygga-1
```

## Timestamp

Timestamp is represented with the following formatting:

```
const TIMESTAMP: &str = "%Y-%m-%dT%H_%M_%S%.3f%z";
```

Which will result in the following representation:

```
2023-11-09T11_20_00.097+0200
```

- This timestamp is unique because of fractional seconds (the `%.3f` format qualifier, decimal fraction of a second with fixed length of 3 digits) in case of spurious power cycles which restart the deployment.
- Timestamp is in local time with timezone offset appended to the string.

How to correctly obtain the time in Rust:

```rust
use chrono::prelude::*;

const TIMESTAMP: &str = "%Y-%m-%dT%H_%M_%S%.3f%z";

fn main() {
    let dt = Local::now();
    println!("{}", dt.format(TIMESTAMP));
}
```

How to obtain the time with the `date` command line utility:

```sh
date +%Y-%m-%dT%H_%M_%S.%3N%z
```

## Supported commands

### 1. Import

```
Usage: hp-cli import [Options] --device <DEVICE_PATH> --output <OUTPUT_PATH>

Options:
  -d, --device <DEVICE_PATH>  Path to USB mass storage or SD card                                                    where data will be imported from.
  -o, --output <OUPUT_PATH>   Path to where the directory for imported                                               data will be created and data will be                                                  imported.
  --clean-imported            Runs a clean after import is complete.
  --audio-previews            Generates compressed previews of audio
                              files.
  -h, --help                  Displays help for available commands,                                                  all other arguments are ignored.
  -v, --verbose               Increase CLI verbosity.
```

This command imports data from the given device, creating a new directory for each imported deployment at the specified output path. This command does not remove files from the device.

At the beginning of the import, the CLI lists the deployments it has detected. For each deployment, the size of the deployment folder is printed. A progress bar shows the progress of the import.

Upon successful import of a single deployment, a metadata file is created. CLI prompts user to fill in optional information.

The device is attached to the host computer as USB mass storage. It is assumed that the output data is located in `/output` directory at the root of the device file system. This directory contains a timestamp-named directory for each deployment (one boot from startup to shut down).

```
/output/
    <TIMESTAMP>/
        audio/
            <TIMESTAMP>_audio.wav
            ...
        gps/
            <TIMESTAMP>_gps.json
            ...
        depth/
            <TIMESTAMP>_depth.csv
        log/
            <TIMESTAMP>_journalctl.txt
    <TIMESTAMP>/
        ...
```

All deployments are imported. The idea is that each deployment would be followed by an import and a cleanup that deletes that deployment from the device. However, one outing with the Hydrophonitor could result in several device restart cycles, so we need to support importing several deployments.

An error message will be printed and non-zero exit code returned in case of an error.

#### Data Formats

##### Audio

The audio is recorded in batches (this is done to avoid data corruption in case of an ungraceful shutdown) as wav files in `/output/<timestamp>/audio` directory and are merged upon import into one .wav file.

##### GPS Data

At the moment of writing, hydrophonitor-gps module records all available data points introduced by gpsd in json files in ``/output/<timestamp>/gps` directory. All json files are merged into one json file upon import.

##### Depth Data

At the moment of writing, depth-recorder module records depth measurements in a csv file in `/output/<timestamp>/depth` directory. If there are multiple csv files, those are merged into one file upon import.

##### Logs

During the deployment, journalctl logs are periodically exported to a text file in `/output/<timestamp>/log` directory.

##### Metadata

When importing a dataset from a deployment, the CLI interface asks the user to fill out optional deployment info. Some of the fields are inferred from the data itself. This data will be saved as `meta.json` file.

```rust
struct DeploymentInfo {
    name: Option<String>,
    tags: Option<Vec<String>>,
    notes: Option<String>,
    start: chrono::DateTime<Local>, // Inferred from the timestamp of first audio file.
    end: chrono::DateTime<Local>, // Inferred from the timestamp and duration of last audio file.
}
```

#### Imported Data Directory Structure

```
<TIMESTAMP>/
    metadata.json
    audio-<TIMESTAMP>-<TIMESTAMP>.wav
    gps-<TIMESTAMP>-<TIMESTAMP>.csv
    depth-<TIMESTAMP>-<TIMESTAMP>.csv
    journalctl-<TIMESTAMP>.log
```

### 2. Clean

```
Usage: hp-cli clean [OPTIONS] --device <DEVICE>

Options:
  -d, --device          Path to USB mass storage or SD card where data                                         will be deleted from.
  -h, --help            Displays help for available commands. All                                              other arguments are ignored.
  -v, --verbose         Increase CLI verbosity.
```

This command removes all deployment data from the given device's `/output` path. Before starting the removal, the CLI displays all deployments it has detected and prompts the user to confirm that these deployments will be deleted.

An error message will be printed and non-zero exit code returned in case of an error.

### 3. Update

```
Usage: hp-cli update [OPTIONS] --device <DEVICE>

Options:
  -d, --device          Path to USB mass storage or SD card where data                                         will be deleted from.
  -h, --help            Displays help for available commands. All                                              other arguments are ignored.
  -v, --verbose         Increase CLI verbosity.
```

### 4. Flash

```
Usage: hp-cli flash [OPTIONS] --device <DEVICE>

Options:
  -d, --device          Path to USB mass storage or SD card where data                                         will be deleted from.
  -h, --help            Displays help for available commands. All                                              other arguments are ignored.
  -v, --verbose         Increase CLI verbosity.
```

### 5. Info

```
Usage: hp-cli info [OPTIONS] --device <DEVICE>

Options:
  -d, --device          Path to USB mass storage or SD card where data                                         will be deleted from.
  -h, --help            Displays help for available commands. All                                              other arguments are ignored.
  -v, --verbose         Increase CLI verbosity.
```

Example:

```
hp info ~/brygga-1
MODEL: Raspberry Pi 4
SOUND_CARD: Scarlett USB 2i2
DEPLOYMENTS: 2023-11-09T11_55_43.007+0200, 2023-11-09T11_55_53.892+0200
```

### 6. Debug

```
Usage: hp-cli debug [OPTIONS] --device <DEVICE>

Options:
  --import-raw <OUTPUT_PATH>    Import output folder from the device                                                   as is.
  -d, --device          Path to USB mass storage or SD card where data                                         will be deleted from.
  -h, --help            Displays help for available commands. All                                              other arguments are ignored.
  -v, --verbose         Increase CLI verbosity.
```

