# Performance Comparison
## test data
1418 dicoms downloaded from [Kaggle dataset](https://www.kaggle.com/datasets/humanaizedata/chest-ct-scans-1m-dicom-files-reports)

## test environment
- Apple M1 Pro
- 32GB RAM
- macOS 14.7.7 (23H723)
## performance comparison (details below)
| Metric | Rust (9 threads) | Rust (1 thread) | Python (no parallel) | Diff (Rust-1 → Py) | Diff (Rust-9 → Py) |
|--------|------------------|-----------------|-----------------------|---------------------|---------------------|
| **Real time (s)** | 1.06 | 2.74 | 4.64 | +1.90 | +3.58 |
| **User time (s)** | 2.07 | 1.60 | 1.64 | +0.04 | –0.43 |
| **Sys time (s)** | 0.66 | 0.46 | 0.56 | +0.10 | –0.10 |
| **Max resident set size (MB)** | 1,204.1 | 911.9 | 1,177.3 | +265.4 | –26.8 |
| **Page reclaims** | 96,482 | 95,126 | 101,581 | +6,455 | +5,099 |
| **Page faults** | 0 | 112 | 76 | –36 | +76 |
| **Voluntary context switches** | 3,004 | 3,177 | 4,906 | +1,729 | +1,902 |
| **Involuntary context switches** | 31,417 | 3,074 | 2,536 | –538 | –28,881 |
| **Instructions retired** | 32,883,935,819 | 28,450,186,221 | 22,997,574,028 | –5,452,612,193 | –9,886,361,791 |
| **Cycles elapsed** | 8,171,193,026 | 6,350,710,295 | 6,750,572,636 | +399,862,341 | –1,420,620,390 |
| **Peak memory footprint (MB)** | 1,568.9 | 1,534.8 | 1,569.7 | +34.9 | +0.8 |

Speed: Rust (9 threads) is the clear winner — 1.06s vs. 4.64s (Python). Nearly 4.4× faster.

CPU usage: Multi-threaded Rust increases user + sys times (work distributed across cores), but wall time plummets.

Memory:

Rust-9 uses ~1.2 GB RSS, slightly above Python’s 1.18 GB.

Peak memory is nearly identical between Rust-9 and Python (~1.57 GB).

Context switches: Rust-9 incurs huge involuntary context switches (31K) vs. Rust-1 (3K) and Python (2.5K). This is expected from thread scheduling overhead.

Efficiency: Rust-9 retired ~33B instructions (more than both Rust-1 and Python), but finished >4× faster than Python, meaning much lower cycles per instruction overall.
## test rust script on all cores
`/usr/bin/time -l ./target/release/fast-dicom-reader read --path /Users/igor/Downloads/anonym/patient11`
```bash
Processing DICOM files in: /Users/igor/Downloads/anonym/patient11
CPU cores detected: 10
Using 9 threads for parallel processing
Found 1418 DICOM files to process

[00:00:00] [########################################] 1418/1418

All DICOM files processed successfully!
Processing completed. Total files: 1418

        1.06 real         2.07 user         0.66 sys
          1204125696  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
               96482  page reclaims
                   0  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                3004  voluntary context switches
               31417  involuntary context switches
         32883935819  instructions retired
          8171193026  cycles elapsed
          1568936256  peak memory footprint
```
## test rust script on a single core
`/usr/bin/time -l ./target/release/fast-dicom-reader read --path /Users/igor/Downloads/anonym/patient11 --threads 1`
```bash
/usr/bin/time -l ./target/release/fast-dicom-reader read --path /Users/igor/Downloads/anonym/patient11 --threads 1
Processing DICOM files in: /Users/igor/Downloads/anonym/patient11
CPU cores detected: 10
Using 1 threads for parallel processing
Found 1418 DICOM files to process
[00:00:02] [########################################] 1418/1418                                                                                           
All DICOM files processed successfully!
Processing completed. Total files: 1418
        2.74 real         1.60 user         0.46 sys
           911851520  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
               95126  page reclaims
                 112  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                3177  voluntary context switches
                3074  involuntary context switches
         28450186221  instructions retired
          6350710295  cycles elapsed
          1534808256  peak memory footprint
```
## test python script (no parallel processing)
```bash
/usr/bin/time -l python /Users/igor/Documents/fast_dicom_reader/fast_dicom_reader/python_dicom_reader/python_dicom_reader/main.py read --path /Users/igor/Downloads/anonym/patient11
Processing DICOM files in: /Users/igor/Downloads/anonym/patient11
Found 1418 DICOM files to process
Processing DICOM files: 100%|█████████████████████████████████████████████████████████████████████| 1418/1418 [00:02<00:00, 600.84file/s]
Processing complete!

All DICOM files processed successfully!
Processing completed. Total files: 1418, Successfully processed: 1418
        4.64 real         1.64 user         0.56 sys
          1177305088  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
              101581  page reclaims
                  76  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                4906  voluntary context switches
                2536  involuntary context switches
         22997574028  instructions retired
          6750572636  cycles elapsed
          1569722752  peak memory footprint
```

