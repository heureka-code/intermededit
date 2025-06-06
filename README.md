Code documentation: [https://heureka-code.github.io/intermededit](https://heureka-code.github.io/intermededit)

Repository: [https://github.com/heureka-code/intermededit](https://github.com/heureka-code/intermededit)

[![Deploy documentation to GitHub Pages](https://github.com/heureka-code/intermededit/actions/workflows/deploy-docs-to-pages.yml/badge.svg)](https://heureka-code.github.io/intermededit)

```shell
❯ cargo doc --open       # open the documentation
```

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

## Finding all connected components

The applied algorithm selects one word from the given list in each step and will
compute all words reachable from this starting point in a maximum of $n$ steps.

If after any of these steps the set of words that should be tried in the next step is empty,
the found words _likely_ form a single and closed connected component of the graph.
In this case the component's words will be stored in a file and removed from the list of words.

If there are after the last iteration still new words that need to be analyzed, it can be assummed that the words are part of a bigger component.
Then these words are stored in another file and get removed from the processed list too so that they get ignored in future steps.

Because of this second part where too big partially computed components get removed it can happen that word groups
that are considered single components in the future could be bigger if they had more words in the list.

**If at least one too big component was found a further analysis is needed in which some components could merge.**

Output for $n=10$.
```shell
Time taken for reading file and precomputing bitmasks of wordlist: 1.791232683s
1816017 complete [01:18:27] [█████████████████████████████████████████████████████████████████████████████████████████████████] 1908795/1908795 (405.4949/s, 0s)
```
With this configuration the program found 828142 single components which are isolated from each other and 369 partials 
that were too big to be sure in just 10 steps.

After moving file and progress bar io to different threads I tried again with $n=20$.
```shell
Time taken for reading file and precomputing bitmasks of wordlist: 1.906969355s
complete: 1805133 in 824647, unknown: 103662 in 7 [01:09:20] [████████████████████████████████████████████████████████████████████████████] 1908795/1908795 (458.8161/s, 0s)
```

This time there were only 7 partial components but also fewer complete components (due to the reason described above).

With $n=42$ no too big word groups were found so these results should be accurate so there are 824294 connected components in the graph created by the given list.
```shell
complete: 1908795 in 824294, unknown: 0 in 0 [01:24:14] [███████████████████████████████████████████████████████████████████████████████] 1908795/1908795 (377.6536/s, 0s)
```

I consider setting a large $n$ (from now on I'll set the maximum representable value in the source code)
being a good alternative to the need of merging single components afterwards.
Eventhough it could be an optimization for larger graphs.

($n=$`usize::MAX`, also print tasks)
```shell
Time taken for reading file and precomputing bitmasks of wordlist: 1.82621978s
(3 steps) BIER -> EIER -> EBER -> LEBER
(5 steps) RHEIN -> REIN -> REN -> RAN -> RAU -> RAUS
(4 steps) EIS -> AIS -> ALS -> ALT -> KALT
(5 steps) BLAU -> BLAUE -> LAUE -> LAUGE -> AUGE -> ALGE
(6 steps) HERZ -> HER -> EHER -> EHEN -> REHEN -> RAHEN -> RASEN
Time taken for completing the tasks (time for creating wordlist excluded): 294.813079ms
complete: 1908795 in 824294, unknown: 0 in 0 [01:11:00] [█████████████████████████████████████████████████████████████████████████████████] 1908795/1908795 (448.0607/s, 0s)
```

With this result we can look at the distribution of words in connected components:
```shell
❯ awk -F '\t' '{print NF}' single-components-maxint.txt  | sort -n | uniq -c
 436948 1
 187214 2
  71806 3
  61140 4
  19600 5
  28690 6
   2580 7
   2687 8
   1246 9
   1209 10
   2386 11
   2009 12
    595 13
    647 14
    930 15
    588 16
    662 17
   1000 18
    282 19
    156 20
    161 21
    180 22
    150 23
    158 24
     93 25
     78 26
     70 27
     97 28
     76 29
     80 30
     49 31
     45 32
     46 33
     44 34
     48 35
     62 36
     23 37
     25 38
     21 39
     21 40
     12 41
     21 42
     24 43
     18 44
     20 45
      7 46
     10 47
     16 48
     20 49
      9 50
     10 51
      9 52
     12 53
     12 54
      5 55
      6 56
     12 57
      7 58
      4 59
      3 60
      9 61
      1 62
      9 63
      3 65
      9 66
      4 67
      2 68
      2 69
      4 70
      2 71
      4 72
      3 73
      6 75
      4 76
      3 78
      3 80
      1 81
      1 82
      2 83
      1 84
      2 85
      1 86
      1 87
      2 88
      2 89
      1 90
      4 91
      2 93
      4 94
      1 95
      1 97
      1 98
      1 99
      1 100
      1 101
      2 102
      1 104
      2 106
      1 107
      2 110
      2 113
      1 114
      1 116
      1 119
      2 121
      2 122
      1 124
      1 129
      1 130
      1 134
      1 135
      3 138
      2 142
      1 143
      1 146
      3 147
      1 150
      1 153
      1 154
      1 159
      1 160
      1 170
      1 172
      1 179
      1 186
      1 191
      1 200
      1 214
      1 228
      1 258
      1 284
      1 288
      1 306
      1 311
      1 344
      1 351
      1 373
      1 428
      1 564
      1 835
      1 105218
```

One component is very large but a lot are smaller and many groups consist of 1 element each.

