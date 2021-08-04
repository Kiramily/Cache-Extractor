# Cache extractor
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Build](https://github.com/Kiramily/Cache-Extractor/actions/workflows/build.yml/badge.svg)

Extract all files from electron Application's cache

## Supported Applications
* [Discord](https://discord.com/)
* [Visual Studio Code](https://code.visualstudio.com/) 
* [Guilded](https://guilded.gg)
* Or Specify a Path

## Usage
```bash
$ cache-extractor <args>
```

### Example 
```bash
$ cache-extractor --app discord --output-dir "./discord-cache"
```

## Arguments
| Argument           | required | value  | default         |
|--------------------|----------|--------|-----------------|
| --app \| -a        | true     | Application | None            |
| --input \| -i      | false    | String | None            |
| --output-dir \| -o | false    | String | %cwd%/extracted |
| --clear-cache \| -c| false	| bool   | false		   |

| Application |
|-------------|
| discord     |
| vscode      |
| vscode-insider|
| guilded     |
| custom (needs input argument) |
