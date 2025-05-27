## First version using subtraction of b'A' from u8 repr of characters
```bash
❯ cargo build --release       # compile binary
...
❯ time cargo run --release -q
(6 steps) HERZ -> HARZ -> HART -> HAST -> HASE -> HASEN -> RASEN
(3 steps) BIER -> EIER -> EBER -> LEBER
(5 steps) BLAU -> BLAUE -> LAUE -> LAUGE -> AUGE -> ALGE
(5 steps) RHEIN -> REIN -> RAIN -> RAN -> RAU -> RAUS
(5 steps) EIS -> EMS -> AMS -> AMT -> KAMT -> KALT

real    0m4,177s
user    0m3,744s
sys     0m0,402s
```

## Other computation that is more aware of diacritic characters with parallel running of the five tasks
```bash
❯ cargo build --release       # compile binary
...
❯ time cargo run --release -q
Time taken for reading file and precomputing bitmasks of wordlist: 1.953297963s
(3 steps) BIER -> EIER -> LEIER -> LEBER
(4 steps) EIS -> AIS -> ALS -> ALT -> KALT
(5 steps) RHEIN -> HEIN -> HAIN -> HAIS -> HAUS -> RAUS
(5 steps) BLAU -> BLAUE -> LAUE -> LAUGE -> AUGE -> ALGE
(6 steps) HERZ -> HETZ -> HATZ -> RATZ -> RATE -> RATEN -> RASEN
Time taken for completing the tasks (time for creating wordlist excluded): 434.710803ms

real    0m4,145s
user    0m4,155s
sys     0m0,388s
```
