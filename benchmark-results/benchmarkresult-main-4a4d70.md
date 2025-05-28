```shell
❯ cargo bench -- --baseline main
     Running benches/max_depth_way_generation.rs (target/release/deps/max_depth_way_generation-7b5538366cfbcab9)
generate-ways/HERZ 5    time:   [222.05 ms 222.69 ms 223.59 ms]
                        change: [-7.3224% -5.4234% -3.6209%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  11 (11.00%) high severe
generate-ways/EIS 5     time:   [450.82 ms 452.10 ms 453.67 ms]
                        change: [-0.6655% -0.2108% +0.2706%] (p = 0.38 > 0.05)
                        No change in performance detected.
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) high mild
  12 (12.00%) high severe
generate-ways/BLAU 5    time:   [223.81 ms 225.31 ms 227.17 ms]
                        change: [+0.1850% +0.9958% +2.0033%] (p = 0.01 < 0.05)
                        Change within noise threshold.
Found 17 outliers among 100 measurements (17.00%)
  4 (4.00%) high mild
  13 (13.00%) high severe

     Running benches/shortest_path.rs (target/release/deps/shortest_path-6188eb2abb84627d)
shortest-path/HERZ -> RASEN
                        time:   [297.81 ms 308.75 ms 320.26 ms]
                        change: [-3.1173% +1.8093% +6.8575%] (p = 0.45 > 0.05)
                        No change in performance detected.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
shortest-path/BIER -> LEBER
                        time:   [3.9238 ms 3.9583 ms 3.9936 ms]
                        change: [-0.4791% +1.8628% +4.5328%] (p = 0.15 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe
shortest-path/BLAU -> ALGE
                        time:   [122.88 ms 128.48 ms 134.10 ms]
                        change: [-11.549% -5.7616% +0.3507%] (p = 0.07 > 0.05)
                        No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
shortest-path/RHEIN -> RAUS
                        time:   [61.200 ms 63.078 ms 64.987 ms]
                        change: [-3.5308% +0.8735% +5.5837%] (p = 0.72 > 0.05)
                        No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
shortest-path/EIS -> KALT
                        time:   [66.923 ms 69.574 ms 72.185 ms]
                        change: [-9.3262% -4.1338% +1.7089%] (p = 0.16 > 0.05)
                        No change in performance detected.

     Running benches/wordlist_read.rs (target/release/deps/wordlist_read-68cabc9cb0a39f8f)
wordlist/bitmasks       time:   [398.78 ms 399.62 ms 400.60 ms]
                        change: [-0.0206% +0.4130% +0.8186%] (p = 0.05 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  4 (4.00%) high mild
  11 (11.00%) high severe
Benchmarking wordlist/io and bitmasks: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 60.0s. You may wish to increase target time to 302.2s, or reduce sample count to 10.
wordlist/io and bitmasks
                        time:   [2.7322 s 2.7407 s 2.7498 s]
                        change: [-0.1015% +0.3201% +0.7641%] (p = 0.15 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
```
