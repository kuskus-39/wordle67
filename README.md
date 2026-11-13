# Wordle67 - a simple art solver for Wordle!

## Usage
```
Usage: wordle67 [OPTIONS] <COMMAND>

Commands:
  solve  Solves the drawing for a specific Wordle answer
  test   Checks all words for compatibility with the drawing and returns a count
  help   Print this message or the help of the given subcommand(s)

Options:
  -d, --drawing <FILE>  [default: examples/67.in]
  -h, --help            Print help
  -V, --version         Print version
```

## Drawing format
Drawings are text files of width 5 and height from 1 to 6, using only characters `-` (black) and `G` (green). Yellow squares are not yet supported.

## Attributions
Wordle-compatible dictionary taken from [dracos/valid-wordle-words.txt](https://gist.github.com/dracos/dd0668f281e685bad51479e5acaadb93).