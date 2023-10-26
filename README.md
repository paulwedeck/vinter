# Vinter Benchmarks

Requirements:
- Java 17
- Gnuplot
- bc
- All dependencies for Vinter (see [https://github.com/KIT-OSGroup/vinter/blob/master/README.md](https://github.com/KIT-OSGroup/vinter/blob/master/README.md))
- While we are not aware of any missing item, this list might be incomplete depending on your system

The benchmark pipeline consists of three steps:
- `./create-files.sh` This script downloads and builds various Vinter versions
- `./run-tests.sh`  This script actually executes the benchmarks (see the results folder)
- `./analyze-results.sh` This script analyzes the raw results and generates plots (see the stats folder)

TESTS.md gives an overview on the various benchmarks.
The following generated figures have been used in the Bachelor's Thesis: 
stats/res1/normalplot.pdf, stats/res8/crashplot.pdf, stats/comb\_bars2.pdf, stats/res11/bars.pdf, stats/panda\_output.pdf

