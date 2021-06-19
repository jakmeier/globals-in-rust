
Each run contains a loop with 10'000 accesses to 3 different global variables.
Reported are averages of 100 samples with lower and upper confidence interval (95%).

You can run your own, simply clone the repo and type `cargo bench`.

Pull requests adding your own runs are welcome, if they add value. (E.g. using a different OS)

# Results on Ubuntu x86_64
* AMD Ryzen 5 3600 6-Core Processor
* 20.04.1-Ubuntu
* rustc 1.51.0 (2fd73fabe 2021-03-23)


## With Interior Mutability
    tls_3                   time:   [21.267 us 21.277 us 21.289 us]
    std_once 3              time:   [266.41 us 267.60 us 268.98 us]
    once_cell 3             time:   [271.15 us 272.00 us 272.82 us]
    lazy_static 3           time:   [271.26 us 271.97 us 272.73 us]
    atomic 3                time:   [123.52 us 123.62 us 123.75 us]
    atomic_seq 3            time:   [123.39 us 123.41 us 123.43 us]

## Read Only
    tls_3 #2                time:   [11.769 us 11.798 us 11.836 us]
    std_once 3 #2           time:   [11.726 us 11.740 us 11.767 us]
    once_cell 3 #2          time:   [7.1753 us 7.2037 us 7.2389 us]
    lazy_static 3 #2        time:   [10.828 us 10.855 us 10.889 us]
    atomic 3 #2             time:   [7.0748 us 7.0898 us 7.1140 us]
    atomic_seq 3 #2         time:   [7.0180 us 7.0188 us 7.0197 us]
