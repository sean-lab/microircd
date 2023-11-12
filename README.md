# microircd
Minimal IRC daemon in Rust

## Description
microircd is a minimal implementation of an IRC daemon written in Rust. 

## How to run microircd in dev env
Run the following command to run microircd in a development environment:

```
cargo run
```


### Adjusting log level
You can adjust the log level of microircd to see more or less output in your console. By default, `env_logger` captures error logs. To change the log level, use one of the following commands based on your operating system and shell before running the server:

#### bash
To set the log level to `info` in bash, use:


```
export RUST_LOG=info
```

#### Powershell
To set the log level to `debug` in Powershell, use:
```
$env:RUST_LOG="debug" 
```

#### Windows Command Prompt
To set the log level to `debug` in the Command Prompt, use:

```
set RUST_LOG=debug
```


## How to build microircd
To compile microircd into an executable, use the following command:


```
cargo build
```


This will produce an executable in the `target/debug` directory for a development build. For a release build with optimizations, use:

```
carbo build --release
```

This will produce an executable in the `target/release` directory.

## Contributing
Contributions to microircd are welcome! Please submit issues and pull requests through GitHub, and ensure that your code adheres to the project's code style guidelines.

## License
microircd is released under the [MIT License](LICENSE).


# TODO 
- Add Unit Tests
- Server to server
- and so on ...
