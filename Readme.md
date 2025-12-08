# AOC 2025

Running the tests requires setting your aoc session cookie in the COOKIE environment variable (or in a .env file)

## Perfs

Benchmark run on a ryzen 5 5600 \
Every solution is single-threaded \
Input reading to a &\[u8\] is not timed in the benchmark \
The input array is not mutable \
The solutions should work for synthetic input containing any skewd distribution or edge case. \
No assumptions are made on the input outside of what can be deduced from the problem description.*

```c
day01 part1:       7,660.06 ns/iter (+/- 420.22)
day01 part2:      12,749.92 ns/iter (+/- 1,241.38)
day02 part1:       1,949.59 ns/iter (+/- 107.77)
day02 part2:      33,163.99 ns/iter (+/- 849.94)
day03 part1:      18,954.57 ns/iter (+/- 460.38)
day03 part2:      40,818.49 ns/iter (+/- 3,058.02)
day04 part1:     103,617.58 ns/iter (+/- 9,372.54)
day04 part2:     940,086.10 ns/iter (+/- 87,931.74)
day05 part1:      20,182.97 ns/iter (+/- 143.47)
day05 part2:       5,052.98 ns/iter (+/- 972.69)
day06 part1:      35,598.26 ns/iter (+/- 1,633.70)
day06 part2:      11,971.42 ns/iter (+/- 604.63)
day07 part1:       2,899.06 ns/iter (+/- 737.68)
day07 part2:       3,387.71 ns/iter (+/- 374.91)
day08 part1:     935,747.15 ns/iter (+/- 33,170.66)
day08 part2:   5,533,428.60 ns/iter (+/- 479,198.98)
```

*see day 8 part 2

## Todo

Change day 4 part 2 to not iterate over and over on the entire grid
