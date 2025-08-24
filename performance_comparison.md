# Performance Comparison
## test data
1418 dicoms downloaded from [Kaggle dataset](https://www.kaggle.com/datasets/humanaizedata/chest-ct-scans-1m-dicom-files-reports)

## test environment
- Apple M1 Pro
- 32GB RAM
- macOS 14.7.7 (23H723)

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
/usr/bin/time -l python /Users/igor/Documents/python_dicom_reader/python_dicom_reader/main.py read --path /Users/igor/Downloads/anonym/patient11
Processing DICOM files in: /Users/igor/Downloads/anonym/patient11
Found 1418 DICOM files to process
Processing DICOM files: 100%|███████████████████████████████████████████████| 1418/1418 [00:02<00:00, 646.61file/s]
Processing complete!

All DICOM files processed successfully!
Processing completed. Total files: 1418, Successfully processed: 1418
        2.38 real         1.31 user         0.32 sys
            72892416  maximum resident set size
                   0  average shared memory size
                   0  average unshared data size
                   0  average unshared stack size
                6842  page reclaims
                  17  page faults
                   0  swaps
                   0  block input operations
                   0  block output operations
                   0  messages sent
                   0  messages received
                   0  signals received
                4893  voluntary context switches
                2027  involuntary context switches
         18416046660  instructions retired
          5102259469  cycles elapsed
            41305472  peak memory footprint
```