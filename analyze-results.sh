#!/bin/bash
cd stats
./analyze_allscript ../results/test1 res1
./analyze_allscript ../results/test4 res4
./analyze_allscript ../results/test8 res8
./analyze_allscript ../results/test9 res9
./analyze_file ./simple_res ../results/test3 ../results/test5 ../results/test6 ../results/test7 ../results/test12 ../results/test14 ../results/test17
./analyze_mod1 ../results/test10 res10
./analyze_mod1 ../results/test11 res11

./analyze2_cmpall res1/summary res4/summary cmp_res
./analyze2_cmpall res10/extract_summary res11/extract_summary cmp_extract
./plot_comb_bar
./plot_comb_bar2
./plot_simple
