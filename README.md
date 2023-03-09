# ambi_mock_client

Provides a mock Ambi client that emulates real sensor hardware such as an Edge client.

## Usage

You must have Rust installed to build `ambi_mock_client`.
You can find documentation on installing Rust [here](https://www.rust-lang.org/tools/install).

You will also need a local copy of [Ambi](https://github.com/jhodapp/ambi) running ( default port 4000 ).

## Set Up Git Hooks

The ambi_mock_client repository makes use of several Git hooks to ensure that code quality standards are met and consistent. To automatically configure these hooks for your local workspace, you can run the following:
```bash
./scripts/create-git-hooks
```

This will create symlinks to the Git hooks, preserving any hooks that you may have already configured.

## Usage

You can either install this as a CLI tool by running `cargo install --path .` (which is the
recommended way of using this) or you can run it directly from this repository.

```
# Installed
$ ambi_mock_client -h

# Or not Installed
$ cargo run -- -h

# Output
Provides a mock Ambi client that emulates real sensor hardware such as an Edge client.

Usage: ambi_mock_client [OPTIONS]

Options:
  -d, --debug
          Turns verbose console debug output on
  -n, --post-amount <POST_AMOUNT>
          The number of sensor readings to post. [DEFAULT: 1]
  -t, --time-per-post <TIME_PER_POST_S>
          The time between each sensor reading post (in seconds). [DEFAULT: 10]
  -T, --total-time <TOTAL_TIME_S>
          The total time over which all the sensor reading posts must be sent (in seconds, alternative to -t)
  -p, --num-threads <NUM_THREADS>
          The number of threads to spawn. The workload will be cloned to each thread, not divided. [DEFAULT: 1]
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
```

Example output:

```log
$ ambi_mock_client -n 1

[2023-03-08T17:39:54Z INFO  ambi_mock_client::sensor_posts] [Thread 0]: Sending POST request to http://localhost:8000/api/readings/add
[2023-03-08T17:39:54Z INFO  ambi_mock_client::sensor_posts] [Thread 0]: Response from Ambi backend: 201
```
