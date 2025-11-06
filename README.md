# Pirate Solitaire Solver

Solver for the [Pirate Solitaire](https://github.com/Pheonyxior/Pirate-Solitaire-Git-Repo) computer card game.

Implemented using A* search algorithm.

For usage information consult with `cargo run --release -- help`.

## Examples

### Solve by seed

`cargo run --release -- seed _3662067111875154814`

It prints to stdout all the games states you can follow to reach the solved state. Beware that the card stacks might get mixed up in some steps, as they are somewhat normalized during solving to prevent visiting repeated game states.

<details>
  <summary>See command output</summary>

```
 6b 5b  s 2t 2r 1b

 1r 6t 4r  t 4b  p

 9t  p 5t 5r 1t 3r

 3t 9r 2b 7r  p 6r

  p  p 8b 4t  w 8r

 3b 7b 9b  p 8t 7t



==============

STEP 0:




 6b 5b  s 2t 2r 1b

 1r 6t 4r  t 4b  p

 9t  p 5t 5r 1t 3r

 3t 9r 2b 7r  p 6r

  p  p 8b 4t  w 8r

 3b 7b 9b  p 8t 7t



==============

STEP 1:




 6b 5b  s 2t 2r 1b

 1r 6t 4r  t 4b  p

 9t  p 5t 5r 1t 3r

 3t 9r 2b 7r  p 6r

  p  p 8b 4t  w 8r

 3b 7b 9b  p    7t

       8t

... more steps ...

==============

STEP 48:

 ## ## ## 8b 9r 9t


 9b



==============

STEP 49:

 ## ## ## 9b 9r 9t

```

</details>


### Solve by card placements

Each line is a card stack, it's transposed if you compare it with the game view.

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

