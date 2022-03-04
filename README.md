# flashtextr
rust implementation of Flashtext

https://pypi.org/project/flashtextr/

## Introduction

This project is a reimplementation of the [Flashtext Algorithm](https://arxiv.org/abs/1711.00046). It is based on the python code found [here](https://github.com/vi3k6i5/flashtext), many thanks vi3k615 for the inspiration!

The key idea behind Flashtext is the trie dictionary, which allows searching for N keywords in a document of length M in O(M) time. This is a significant performance improvement over the O(NxM) time for regex.

## Useful Commands:

- `cargo test` : run the test suite.
- `maturin develop --release` : build a python library with the rust backend to the local python environment.

## Python bindings:

- `Flashtext` : initialize a new processor `x = Flashtext(case_sensitive: Bool)`
- `x.add_keyword()`: add a new keyword
- `x.has_keyword()`: checks if a keyword is in dict
- `x.extract_keywords()`: finds instances of keywords in a string.

## Performance:

Keywords  | FlashText | Regex | Flashtextr
---|---|---|---
0      | 0.07697   | 0.06110   | 0.00592   
1000   | 0.09874   | 0.47255   | 0.01027   
2000   | 0.09982   | 0.71951   | 0.01143   
3000   | 0.10395   | 1.05975   | 0.01209   
4000   | 0.10857   | 1.42789   | 0.01307   
5000   | 0.14982   | 1.78504   | 0.01414   
6000   | 0.11272   | 2.19296   | 0.01477   
7000   | 0.11959   | 2.51350   | 0.01630   
8000   | 0.11690   | 2.90644   | 0.01648   
9000   | 0.11807   | 3.21026   | 0.01675   
10000  | 0.11621   | 3.60510   | 0.01778   
11000  | 0.11955   | 3.97514   | 0.01819   
12000  | 0.12442   | 4.29205   | 0.01906   
13000  | 0.12256   | 4.61641   | 0.01928   
14000  | 0.12181   | 5.02390   | 0.02056   
15000  | 0.12476   | 5.48964   | 0.02140   
16000  | 0.12742   | 5.61275   | 0.02166   
17000  | 0.12959   | 6.03048   | 0.02302   
18000  | 0.13091   | 6.33640   | 0.02339   
19000  | 0.13218   | 6.65673   | 0.02372   
20000  | 0.13322   | 6.98852   | 0.02477   
