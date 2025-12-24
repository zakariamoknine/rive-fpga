#!/bin/bash

if [ ! -d "submodules/VexiiRiscv" ]; then
    echo "ERROR: submodules/VexiiRiscv not found. Ensure you're running this script from project's root." >&2
    exit 1
fi

cd submodules/VexiiRiscv

#
# CONFIG: rv64imac_zicsr_zifencei_zicbom, MMU, S-mode
#
sbt "Test/runMain vexiiriscv.Generate
	--xlen 64 \
	--reset-vector 0x00004000 \
	--with-mul --with-div --with-rva --with-rvc \
	--with-rvZcbm \
	--with-supervisor --with-user \
	--pmp-size=4 --pmp-granularity=4096 \
	--allow-bypass-from=0 --performance-counters=0 \
	--lsu-l1 --lsu-axi4 --lsu-l1-axi4 --lsu-l1-ways=2 --lsu-l1-mem-data-width-min=64 --with-lsu-bypass \
	--fetch-l1 --fetch-axi4 --fetch-l1-ways=2 --fetch-l1-mem-data-width-min=64 \
	--with-btb --with-ras --with-gshare \
	--with-rdtime \
	--region base=00000000,size=F000000,main=1,exe=1 \
	--region base=80000000,size=8000000,main=1,exe=1 \
	--region base=10000000,size=6F000000,main=0,exe=0"
