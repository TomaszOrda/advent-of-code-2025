A record of my solutions for 2025 [Advent Of Code](https://adventofcode.com/2025 "AoC").

Notice, that since this year the event consists of only 12 days.

Solutions for each day and task are in separate files. Sumbmited (unoptimized) solutions are placed in `\src\Old\`.

To run the solution, run the whole project `cargo run x y` setting `x` as day and `y` task number. Most solutions include `#[test]` functions, with AoC examples and additional tests. Those can be all run through `cargo test`. There is third optional `suffix` parameter that will be appended to the input data file name. One can use it instead of `cargo test` functionality for manual testing.

Data for each task can be accessed through the site. It should be placed in the data folder with name `dayx.txt`, where `x` is the day number. Additional test data should have the same name, but with an additional suffix, for example `dayxtest.txt`.

All solutions but one top out at about a second on my machine in debug mode, and tenth of that in relase mode. Day 10 part 2 needs about 40s in release mode. My previous approach (18 times slower) was completely different from canonical, so I allowed myself to use a hint to what a canonical solution would be (after solving the puzzle the hard way).

24/24⭐
|            | 月 | 火 | 水 | 木 | 金 | 土 | 日 |
|:-----------|-------|-------|-------|-------|-------|-------|-------|
| **Week 1** |⭐&nbsp;⭐|⭐&nbsp;⭐|⭐&nbsp;⭐|⭐&nbsp;⭐|⭐&nbsp;⭐|⭐&nbsp;⭐|⭐&nbsp;⭐|
| **Week 2** |⭐&nbsp;⭐|⭐&nbsp;⭐|⭐&nbsp;⭐|⭐&nbsp;⭐|⭐&nbsp;🌟|||

An estimated rundown of the time invested in each day puzzle (includes coding and coding-adjacent activities). Bar sections represent from the leftmost: first part solution, second part solution, solutions refinement and utilities.

```
Prep   | ||█████████████████████████████ 252

Day  1 | █████|███████| 104
Day  2 | ███████████████████|█████| 206
Day  3 | ███|███| 51
Day  4 | ███████|█|██ 90
Day  5 | ███|███|█████ 94
Day  6 | ████|██████|████ 118
Day  7 | ███|██|█ 50
Day  8 | ██████████|████|█████ 165
Day  9 | █|██████████████|████ 165
Day 10 | ████████|█████████████████████████████████████████████████████████| 568
Day 11 | ██|███| 40
Day 12 | █████████||██ 98
```
<!-- 33 hours 23 minutes total, with some change I did not count like writing this sentence.>