# Pirate Solitaire Solver

Solver for the [Pirate Solitaire](https://github.com/Pheonyxior/Pirate-Solitaire-Git-Repo) computer card game.

Implemented using A* search algorithm.

For usage information consult with `cargo run --release -- help`.

## Examples

### Solve by seed

`cargo run --release -- seed _3662067111875154814`

### Solve by card placements

Each line is a card stack, it's transposed comparing with the game view.

```sh
cargo run --release -- cards /dev/stdin <<EOM
4r 1t 1r 7r p 9b
p 3t 2t 5b w 6t
8t 7b 3b s 6b 5r
4t p 5t p 9r p
8r 8b 4b 1b 9t t
2r p 6r 3r 2b 7t
EOM
```

