# letters

`letters` does stuff relating to letters. Stuff like counting them, checking if they
exist, removing puntuation around them, and turning them into json.

Pro tip: `letters` cannot work with words. Use [words](https://github.com/Skyppex/words) for that.
Pro pro tip: `letters` cannot work with sentences. Use [words](https://github.com/Skyppex/words) for that too for some reason.

## Installation

### Build from source

- Clone the repo
  `git clone https://github.com/Skyppex/letters.git`
  `gh repo clone Skyppex/letters`

- Build with cargo
  `cargo build --release`

## Usage

Usage: letters.exe [OPTIONS]

Options:

- `-s`, `--source` \<SOURCE\> The source file to read from. If not provided, read from stdin
- `-d`, `--destination` \<DESTINATION\> The destination file to write to. If not provided, write to stdout
- `-f`, `--first` [\<FIRST\>] Get the first n letters from the input. (default 1)
- `-l`, `--last` [\<LAST\>] Get the last n letters from the input. (default 1)
- `-e`, `--equals` \<EQUALS\> Filter the input to only a single character
- `-C`, `--case-sensitive` Case-sensitive matching
- `-L`, `--list` Print the result as a list separated by newlines
- `-j`, `--json` Print the result as a json list
- `-p`, `--no-punctuation` Remove punctuation from the output
- `-t`, `--trim-whitespace` Trim whitespace from the output
- `-w`, `--lowercase` Convert input to lowercase
- `-n`, `--count` Count the number of words in the output
- `-g`, `--group`
- `-h`, `--help` Print help
- `-V`, `--version` Print version

## Contributing

Issues and PRs are welcome!
