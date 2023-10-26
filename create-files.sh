#!/bin/bash
git clone https://github.com/paulwedeck/WineFS --branch vinter
git clone https://github.com/paulwedeck/vinter vinter_base
./create-new-vinter.sh pre
./create-new-vinter.sh pre_mod1
./create-new-vinter.sh opt1
./create-new-vinter.sh opt1_mod1
./create-new-vinter.sh opt2
./create-new-vinter.sh final
