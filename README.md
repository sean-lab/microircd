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

## Docker

### Building the Docker Image

To build a Docker image for microircd, you can use the provided Dockerfile. Here's how you can build the Docker image:

```bash
docker build -t microircd .
```
This command builds a Docker image using the Dockerfile in the current directory and tags the image as microircd.

### Running the Docker Image
After you've built the Docker image, you can run it with the following command:

This command runs the microircd Docker image. The -it option allows you to interact with the container via your terminal, and the --rm option automatically removes the container when it exits.

```bash
docker run -it --rm -p 6667:6667 microircd
```

## Contributing
Contributions to microircd are welcome! Please submit issues and pull requests through GitHub.

## License
microircd is released under the [MIT License](LICENSE).


# TODO 
- Add Unit Tests
- Server to server
- and so on ...
