[![Rust](https://github.com/jhkersul/monty-hall-rust/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/jhkersul/monty-hall-rust/actions/workflows/rust.yml)

# Monty Hall Problem in Rust

This is just a curious implementation of [Monty Hall Problem](https://en.wikipedia.org/wiki/Monty_Hall_problem).

![Monty Hall Problem Representation](https://upload.wikimedia.org/wikipedia/commons/3/3f/Monty_open_door.svg)

The project was written in Rust, to run you just need to run with cargo:

```
$ cargo run
```

The output is the number of plays and comparison between changing doors and not changing:

```
Total Games: 2188 - Won Change Door: 1422 - Won Not Change Door: 726 - Percentage Change Door: 64.99086 - Percentage Not Change Door: 33.18099
```

