# Chess CLI

A small, super basic chess CLI.

This project was written in an hour or two for the heck of it when I realized that there were Unicode chess figures for all the different pieces. It doesn't do move validation, win state, move recording, or really anything except moving pieces around a board.

## Use

I'm not going to build and release this project, so you'll have to use the source code to build it locally.

1. Clone the repo.
2. `cargo run`.
3. Play
   1. Enter moves formatted as `{square to pick up}{square to move to}`. For example on move 1 white might play `e2e4`.
   2. `Ctrl-C` to exit.
   3. Note that a terminal theme with a dark background and light text can cause the chess pieces to look like they're opposite of what they should be. The outlined pieces are white and the solid pieces are black no matter what the theme is.

## Some cool ideas for further development

- Player names
- Time controls
- Move validation
- Move recording and display
- Import a game state from a PGN
- Move to TUI that can be more interactive
