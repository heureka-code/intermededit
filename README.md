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
