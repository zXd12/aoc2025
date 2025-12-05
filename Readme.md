# AOC 2025

Running the tests requires setting your aoc session cookie in the COOKIE environment variable (or in a .env file)

## Perfs

Benchmark run on a 5 3500 \
Every solution is single-threaded \
Input reading to a &\[u8\] is not counted in the time \
The input array is not mutable

```
day01 part1:       9,958.90 ns/iter (+/- 269.57)
day01 part2:      16,034.09 ns/iter (+/- 653.64)
day02 part1:       2,350.06 ns/iter (+/- 134.99)
day02 part2:      49,345.46 ns/iter (+/- 2,394.60)
day03 part1:      26,989.48 ns/iter (+/- 435.90)
day03 part2:      51,104.19 ns/iter (+/- 3,759.63)
day04 part1:     150,092.67 ns/iter (+/- 4,149.15)
day04 part2:   1,231,421.85 ns/iter (+/- 22,271.54)
day05 part1:      29,094.43 ns/iter (+/- 545.26)
day05 part2:       7,065.56 ns/iter (+/- 387.80)
```

## Todo

Change day 4 part 2 to not iterate over and over on the entire grid
