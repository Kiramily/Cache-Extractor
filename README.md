# Discord Cache extractor
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![example workflow](https://github.com/Kiramily/Discord-Cache-Extractor/actions/workflows/build.yml/badge.svg)

Extracts all the Files from discord cache to `%pictures%/extracted`

## Usage
```bash
$ discord_cache_extractor <args>
```

### Example 
```bash
$ discord_cache_extractor --output-dir "./discord-cache"
```

## Arguments
| Argument           | required | value  | default         |
|--------------------|----------|--------|-----------------|
| --output-dir \| -o | false    | String | %cwd%/extracted |
