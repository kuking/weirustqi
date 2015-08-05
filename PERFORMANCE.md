# Performance
The performance is measured using the following logic to generate random games:

- It keeps playing until the game has finished (unlikely) or the move number is
  equals to `board_size^2` (81 moves for 9x9, 121 for 11x11 and 361 for 19x19).
- If it can't find a valid move after `board_size*2` tries, it passes.

The games tend to be played until the move limit is reached.

## First working version
First working version with a fair performant implementation but far from
optimal:

| benchmark        | speed ns/iter                | playouts/sec |
|------------------|------------------------------|--------------|
|play_random_9x9   | 115,742 ns/iter (+/- 21,321) | 8639 p/s     |
|play_random_11x11 | 176,355 ns/iter (+/- 28,005) | 5670 p/s     |
|play_random_19x19 | 562,699 ns/iter (+/- 93,400) | 1777 p/s     |

Measured with an _Intel(R) Core(TM) i7-4558U CPU @ 2.80GHz_
(via `sysctl -n machdep.cpu.brand_string`)

## After some code cleanup and optimisations

| benchmark        | speed ns/iter                | playouts/sec |
|------------------|------------------------------|--------------|
|play_random_9x9   | 66,562 ns/iter (+/- 4,119)   | 15023 p/s    |
|play_random_11x11 | 103,510 ns/iter (+/- 12,822) | 9660 p/s     |
|play_random_19x19 | 318,778 ns/iter (+/- 43,472) | 3136 p/s     |
