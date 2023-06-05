# Hanji

```
Welcome to Hanji, Hanji builds docs for your Cairo code

Usage: hanji [OPTIONS] <PATH> [OUT_DIR]

Arguments:
  <PATH>     Path to the cairo file or directory to parse
  [OUT_DIR]  Path to output the docs in, default hanji-out

Options:
  -i, --index                                  Print the index, can be pasted in readme.md
  -x, --index-path-prefix <INDEX_PATH_PREFIX>  Index links path prefix, defaults to out_dir path
  -h, --help                                   Print help
  -V, --version                                Print version
```

## Installation

1. Clone the repo.
2. Go to the terminal and run the following commands.

    - Build binary

        ```sh
        cargo build --release
        ```

    - Add $PATH to binary, replace `.bash_profile` with `.zshrc` if you are using zsh.

        ```sh
        echo "export PATH=\$PATH:$(pwd)/target/release" >> ~/.bash_profile
        ```

    - Close the terminal and open it again to load updated `PATH`.
    - Verify you can run `hanji -h` or `hanji.exe -h` (for windows)

        ```sh
        hanji -h
        ```

You should see a welcome message, if not please look up how to add `PATH` for your terminal/OS.
