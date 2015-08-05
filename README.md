# WeiRustQi - Computer-GO library [<img src="https://travis-ci.org/kuking/weirustqi.svg?branch=master">](https://travis-ci.org/kuking/weirustqi)
Intended to be fast, optimised and simple library for Computer GO in Rust.
It is a very long shot (the second one for me) on implementing a fast Go library
for supporting Monte Carlo and machine learning approaches.

## Performance
First working version with a fair performant implementation but far from
optimal; measured with: _Intel(R) Core(TM) i7-4558U CPU @ 2.80GHz_
(via `sysctl -n machdep.cpu.brand_string`)

The following values were obtained using `cargo bench`, therefore the binaries
utilised are not optimised.  

| benchmark        | speed ns/iter                | playouts/sec |
|------------------|------------------------------|--------------|
|play_random_9x9   |  115,742 ns/iter (+/- 21,321)| 8639 p/s     |
|play_random_11x11 |  176,355 ns/iter (+/- 28,005)| 5670 p/s     |
|play_random_19x19 |  562,699 ns/iter (+/- 93,400)| 1777 p/s     |
