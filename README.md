# csv2ndjson-lite

A little tool to convert a csv to a valid ndjson/json-stream (supported comma separated arrays).

## Installation

You can use cargo to install this tool:

```bash
cargo install -f csv2ndjson-lite
```

## Usage

Here is an example CSV file, note that there is fields that are comma separated.

```csv
title,genres,scores,duration
Carmencita,"Documentary,Short","23,25,56",3.5
Miss Jerry,Romance,"3,2,6",5.3
Corbett and Courtney Before the Kinetograph,"Short,Sport","203,250,506",
```

If you want to output an array when CSV values are comma separated, specify those headers as arguments.

```bash
cat mydata.csv | csv2ndjson-lite --arrays genres scores --numbers scores duration
```

The output of the previous command would be something like so.

```json
{"title":"Carmencita","genres":["Documentary","Short"],"scores":[23,25,56],"duration":3.5}
{"title":"Miss Jerry","genres":["Romance"],"scores":[3,6],"duration":5.3}
{"title":"Corbett and Courtney Before the Kinetograph","genres":["Short","Sport"],"scores":[203,250]}
```
