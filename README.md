# csv2ndjson-lite

A little tool to convert a csv to a valid ndjson/json-stream (supported comma separated arrays).

## Usage

Here is an example CSV file, note that there is fields that are comma separated.

```csv
title,genres
Carmencita,"Documentary,Short"
Miss Jerry,Romance
Corbett and Courtney Before the Kinetograph,"Short,Sport"
```

If you want to output an array when CSV values are comma separated, specify those headers as arguments.

```bash
cat mydata.csv | cargo run --release 'genres'
```

The output of the previous command would be something like so.

```json
{"title":"Carmencita","genres":"Documentary,Short"}
{"title":"Miss Jerry","genres":"Romance"}
{"title":"Corbett and Courtney Before the Kinetograph","genres":"Short,Sport"}
```
