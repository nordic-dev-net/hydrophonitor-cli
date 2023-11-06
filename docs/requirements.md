## Hydrophonitor CLI

### Requirements

Introduces a command line utility called `hp-cli` which interfaces with a Hydrophonitor, a Raspberry Pi based hydrophone passive acoustic monitoring system.

### Supported commands

#### 1. Import

Introduces the following command:

```sh
hp-cli import DEVICE OUTPUT [options]
```

```
Required arguments:

DEVICE                      Path to USB mass storage or SD card where data will
                            be imported from.

OUTPUT                      Path to where the directory for imported data
                            will be created and data will be imported.

Supported options:

--debug-import              Do not merge any files, but copy everything from the device /output
                            folder to OUTPUT.

--help                      Displays help for available commands. All other arguments are ignored.

--verbose                   Increase CLI verbosity.
```

This command imports data from the given device, creating a new directory for each imported deployment at the specified output path. This command does not remove files from the device.

At the beginning of the import, the CLI lists the deployments it has detected. For each deployment, the size of the deployment folder is printed. A progress bar shows the progress of the import.

Upon successful import of a single deployment, a metadata file is created. CLI prompts user to fill in optional information.

The device is attached to the host computer as USB mass storage. It is assumed that the output data is located in `/output` directory at the root of the device filesystem. This directory contains a timestamp-named directory for each deployment (one boot from startup to shutdown).

```
/output/
    <timestamp>/
        audio/
            <timestamp>_audio.wav
            ...
        gps/
            <timestamp>_gps.json
            ...
        depth/
            <timestamp>_depth.csv
            ...
        log/
            <timestamp>_journalctl.txt
    <timestamp>/
        ...
```

All deployments are imported. The idea is that each deployment would be followed by an import and a cleanup that deletes that deployment from the device. However, one outing with the Hydrophonitor could result in several device restart cycles, so we need to support importing several deployments.

Imported files are processed as described below, unless option `--debug-import` is given. In that case, all files and directories will be imported from `/output` directory as such.

An error message will be printed and non-zero exit code returned in case of an error.

#### 1.1 Data Formats

##### 1.1.1 Audio

The audio is recorded in batches (this is done to avoid data corruption in case of an ungraceful shutdown) as wav files in /output/<timestamp>/audio directory and are merged upon import into one .wav file.

##### 1.1.2 GPS Data

At the moment of writing, hydrophonitor-gps module records all available data points introduced by gpsd in json files in /output/<timestamp>/gps directory. All json files are merged into one json file upon import.

##### 1.1.3 Depth Data

At the moment of writing, depth-recorder module records depth measurements in a csv file in /output/<timestamp>/depth directory. If there are multiple csv files, those are merged into one file upon import.

##### 1.1.4 Logs

During the deployment, journalctl logs are periodically exported to a text file in /output/<timestamp>/log directory.

##### 1.1.5 Metadata

When importing a dataset from a deployment, the CLI interface asks the user to fill out optional deployment info. Some of the fields are inferred from the data itself. This data will be saved as `meta.json` file.

```rs
struct DeploymentInfo {
    name: Option<String>,
    tags: Option<Vec<String>>,
    notes: Option<String>,
    start: chrono::DateTime<Utc>, // Inferred from the timestamp of first audio file.
    end: chrono::DateTime<Utc>, // Inferred from the timestamp of last audio file.
}
```

#### 1.2 Imported Data Directory Structure

```
<timestamp>-<device_name>/
    meta.json
    <timestamp>_audio.wav
    <timestamp>_gps.json
    <timestamp>_depth.csv
    <timestamp>_journalctl.txt
```

#### 2. Clean

Introduces the following command:

```sh
hp-cli clean DEVICE [options]
```

```
Required arguments:

DEVICE                  Path to USB mass storage or SD card where data will
                        be deleted from.

Supported options:

--help                  Displays help for available commands. All other arguments are ignored.

--verbose               Increase CLI verbosity.
```

This command removes all deployment data from the given device's `/output` path. Before starting the removal, the CLI displays all deployments it has detected and prompts the user to confirm that these deployments will be deleted.

An error message will be printed and non-zero exit code returned in case of an error.
