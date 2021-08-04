# Cache extractor
[![License](https://img.shields.io/github/license/Kiramily/Cache-Extractor)](https://opensource.org/licenses/MIT)
![Build](https://github.com/Kiramily/Cache-Extractor/actions/workflows/build.yml/badge.svg)
[![Version](https://img.shields.io/github/v/tag/Kiramily/Cache-Extractor)](https://github.com/Kiramily/Cache-Extractor/releases)

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
| -file-filter \| -f | false    | Regex String | ^f_       |

| Application |
|-------------|
| discord     |
| vscode      |
| vscode-insider|
| guilded     |
| custom (needs input argument) |

## Acknowledgements 
* [mean bean ci template](https://github.com/XAMPPRocky/mean-bean-ci-template)
