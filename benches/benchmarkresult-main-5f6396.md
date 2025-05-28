```shell
❯ cargo bench -- --save-baseline main
     Running benches/max_depth_way_generation.rs (target/release/deps/max_depth_way_generation-7b5538366cfbcab9)
generate-ways/HERZ 5    time:   [231.16 ms 235.46 ms 240.21 ms]
Found 11 outliers among 100 measurements (11.00%)
  7 (7.00%) high mild
  4 (4.00%) high severe
generate-ways/EIS 5     time:   [451.72 ms 453.05 ms 454.67 ms]
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) high mild
  11 (11.00%) high severe
generate-ways/BLAU 5    time:   [222.45 ms 223.09 ms 223.95 ms]
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) high mild
  11 (11.00%) high severe

     Running benches/shortest_path.rs (target/release/deps/shortest_path-6188eb2abb84627d)
shortest-path/HERZ -> RASEN
                        time:   [294.57 ms 303.26 ms 312.06 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
shortest-path/BIER -> LEBER
                        time:   [3.9284 ms 3.9645 ms 3.9993 ms]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe
shortest-path/BLAU -> ALGE
                        time:   [130.16 ms 136.33 ms 142.54 ms]
shortest-path/RHEIN -> RAUS
                        time:   [60.359 ms 62.532 ms 64.767 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
shortest-path/EIS -> KALT
                        time:   [69.324 ms 72.574 ms 75.865 ms]

     Running benches/wordlist_read.rs (target/release/deps/wordlist_read-68cabc9cb0a39f8f)
wordlist/bitmasks       time:   [396.69 ms 397.98 ms 399.44 ms]
Found 14 outliers among 100 measurements (14.00%)
  7 (7.00%) high mild
  7 (7.00%) high severe
Warning: Unable to complete 100 samples in 60.0s. You may wish to increase target time to 342.1s, or reduce sample count to 10.
wordlist/io and bitmasks
                        time:   [2.7246 s 2.7319 s 2.7399 s]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe
```
