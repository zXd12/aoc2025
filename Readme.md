# AOC 2025

Running the tests requires setting your aoc session cookie in the COOKIE environment variable (or in a .env file)

## Perfs

Benchmark run on a ryzen 5 5600 \
Every solution is single-threaded \
Input reading to a &\[u8\] is not timed in the benchmark \
The input array is not mutable

```
day01 part1:       7,818.73 ns/iter (+/- 171.33)
day01 part2:      12,254.26 ns/iter (+/- 650.21)
day02 part1:       1,726.44 ns/iter (+/- 89.20)
day02 part2:      31,818.20 ns/iter (+/- 1,704.90)
day03 part1:      19,289.06 ns/iter (+/- 908.67)
day03 part2:      35,045.39 ns/iter (+/- 2,621.11)
day04 part1:     105,059.22 ns/iter (+/- 4,340.17)
day04 part2:     914,476.25 ns/iter (+/- 15,397.12)
day05 part1:      20,535.00 ns/iter (+/- 1,410.90)
day05 part2:       5,133.24 ns/iter (+/- 140.38)
day06 part1:      38,253.05 ns/iter (+/- 2,720.55)
day06 part2:      11,524.62 ns/iter (+/- 846.51)
day07 part1:       2,772.29 ns/iter (+/- 164.69)
day07 part2:       3,309.69 ns/iter (+/- 93.42)
```

## Todo

Change day 4 part 2 to not iterate over and over on the entire grid
