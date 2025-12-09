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
day 01 part 1:       8,338.64 ns/iter (+/- 491.49)
day 01 part 2:      12,979.36 ns/iter (+/- 3,995.64)
day 02 part 1:       1,964.64 ns/iter (+/- 132.32)
day 02 part 2:      33,163.97 ns/iter (+/- 790.32)
day 03 part 1:      19,041.04 ns/iter (+/- 2,639.38)
day 03 part 2:      34,642.08 ns/iter (+/- 2,573.72)
day 04 part 1:     106,965.77 ns/iter (+/- 2,021.60)
day 04 part 2:     943,503.30 ns/iter (+/- 36,294.49)
day 05 part 1:      21,085.81 ns/iter (+/- 128.17)
day 05 part 2:       5,171.55 ns/iter (+/- 411.52)
day 06 part 1:      38,259.12 ns/iter (+/- 2,975.25)
day 06 part 2:      10,580.47 ns/iter (+/- 267.46)
day 07 part 1:       3,070.94 ns/iter (+/- 139.55)
day 07 part 2:       3,532.54 ns/iter (+/- 191.68)
day 08 part 1:     929,754.22 ns/iter (+/- 21,589.17)
day 08 part 2:   3,450,174.95 ns/iter (+/- 41,488.10)
day 09 part 1:     125,553.66 ns/iter (+/- 1,027.74)
```

*see day 8 part 2

## Todo

Change day 4 part 2 to not iterate over and over on the entire grid
