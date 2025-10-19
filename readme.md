# Cube Type Extractor - rs
![build](https://github.com/mikkurogue/cube-type-extract-rs/actions/workflows/rust.yml/badge.svg) ![build](https://github.com/mikkurogue/cube-type-extract-rs/actions/workflows/rust-clippy.yml/badge.svg) [![Release](https://github.com/mikkurogue/cube-type-extract-rs/actions/workflows/release.yml/badge.svg)](https://github.com/mikkurogue/cube-type-extract-rs/actions/workflows/release.yml)

This is the Rust rewrite of the [Cube Type Extractor](https://github.com/mikkurogue/cube-type-extract) project that was initially written in Go.

The reason for this rewrite is not because of the meme, but because it's a relatively simple and small project to learn Rust with that has some core concepts for programming.

This project will essentially be a 1 to 1 copy so the configuration, commands and everything else should all be the same.

## Configuration

To get started run the following command:

```bash
cube-type-extract-rs --init
```

This will create a `type-gen-config.json` file with the following content:

```json
{
  "url": {
    "link": "http://localhost:4000/cubejs-api",
    "headers": {
      "Authorization": "Bearer your-token-here"
    }
  },
  "output": "./",
  "file_name": "cubejs-types",
  "prefixes": [
    {
      "name": "Placeholder",
      "prefix": "Main"
    }
  ],
  "enable_count_types": true,
  "enable_check_existence_fields": true
}
```

### Options

- `url`: Configuration for the Cube.js API endpoint.
  - `link`: The URL of your Cube.js API. Defaults to `http://localhost:4000/cubejs-api`.
  - `headers`: **(Optional)** Custom HTTP headers to include in the request. Useful for authentication or API keys. Can be omitted if no headers are needed.
- `output`: The output directory for the generated types. Defaults to `./`.
- `file_name`: The name of the generated file. Defaults to `cubejs-types`.
- `prefixes`: A list of prefixes to use for the generated types. This is useful if you have multiple cubes with the same name.
  - `name`: The name of the cube.
  - `prefix`: The prefix to use for the generated types.
- `enable_count_types`: Whether to generate types for the `count` field. Defaults to `true`.
- `enable_check_existence_fields`: Whether to generate types for the `check existence` fields. Defaults to `true`.

**Note:** The `headers` field is optional and can be completely omitted from the configuration if your API doesn't require custom headers. Header values can be strings, numbers, or booleans.

## Commands

### New

To add a new cube to the configuration, you can use the `new` command. This is useful when you want to add a new cube without manually editing the configuration file.

```bash
cube-type-extract-rs new <cube_name> <prefix>
```

- `<cube_name>`: The name of the cube you want to add.
- `<prefix>`: The prefix you want to assign to the cube.

### Remove

To remove a cube from the configuration, you can use the `remove` command. This is useful when you want to remove a cube without manually editing the configuration file.

```bash
cube-type-extract-rs remove <cube_name>
```

- `<cube_name>`: The name of the cube you want to remove.

## Benchmarks

For benchmark results, please see the [benchmark file](./bench.md).