# yamine [![Mean Bean CI](https://github.com/avencera/yamine/workflows/Mean%20Bean%20CI/badge.svg)](https://github.com/avencera/yamine/actions?query=workflow%3A%22Mean+Bean+CI%22)

A simple CLI for combining json and yaml files

## Install

Available via Homebrew/Linuxbrew

`brew install avencera/tap/yamine`

OR

Install from a github release:

`curl -LSfs https://avencera.github.io/yamine/install.sh | sh -s -- --git avencera/yamine`

OR

Install using cargo:

`cargo install yamine` or `cargo binstall yamine`

OR

Download a release directly from github: [github.com/avencera/yamine/releases](https://github.com/avencera/yamine/releases)

## Usage

`yamine --help`

```
Combine JSON/YAML files into a single file

Usage: yamine [OPTIONS] [FILES_OR_FOLDERS]...

Arguments:
  [FILES_OR_FOLDERS]...  File(s) or folder you want to run in

Options:
  -i, --stdin            Read from STDIN
  -d, --depth <DEPTH>    Number of folder depth to recurse into [default: 1]
  -o, --output <OUTPUT>  Output file name [default: combined.yaml]
      --dry-run          Default mode
  -w, --write            Write new output file
  -s, --stdout           Outputs combined file contents to STDOUT
  -f, --format <FORMAT>  The format for the output file, defaults to yaml [default: yaml] [possible values: yaml, json-array, json-k8s, json]
  -h, --help             Print help
  -V, --version          Print version
```

## Examples

- Combine all yaml and json files in the current folder and creates `combined.yaml` file
  - `yamine -w .`
- Combine all yaml and json files in the current folder and creates a `combined.json` file in `json-k8s` format:
  - `yamine --write --format json-k8s --output combined.json .`
- Output the combined file to STDOUT in json format:
  - `yamine --stdout -f json .`
- Convert YAML from stdin and output as JSON to stdout
  - `pbpaste | yamine --stdin --stdout -f json`

## Formats

- `yaml` - a multi document yaml file separated by `---` (a kubernetes multi resource document)

  ```yaml
  ---
  apiVersion: traefik.containo.us/v1alpha1
  kind: IngressRoute
  ---
  apiVersion: v1
  kind: Namespace
  metadata:
    name: example
  ---
  kind: ServiceAccount
  apiVersion: v1
  ---
  apiVersion: v1
  kind: Service
  ---
  apiVersion: apps/v1
  kind: Deployment
  ```

- `json` - a json file with each combined file being an element in the array

  ```json
  [
      {
        "apiVersion": "traefik.containo.us/v1alpha1",
        "kind": "IngressRoute"
        ...
      },
      {
        "apiVersion": "v1",
        "kind": "Namespace",
        ...
      },
      {
        "apiVersion": "v1",
        "kind": "ServiceAccount",
        ...
      },
      {
        "apiVersion": "v1",
        "kind": "Service",
        ...
      },
  ]
  ```

- `json-k8s` - a kubernetes multi resource json document ex:

  ```json
  {
    "kind": "List",
    "apiVersion": "v1",
    "items": [
      {
        "apiVersion": "traefik.containo.us/v1alpha1",
        "kind": "IngressRoute"
        ...
      },
      {
        "apiVersion": "v1",
        "kind": "Namespace",
        ...
      },
      {
        "apiVersion": "v1",
        "kind": "ServiceAccount",
        ...
      },
      {
        "apiVersion": "v1",
        "kind": "Service",
        ...
      },
    ]
  }
  ```
