IMG_SIZE=16777216
VINTER_PATH=./../

ALL_CMDS=$(shell ls *.cmd)

ALL_IMGS=$(ALL_CMDS:.cmd=.img) clean.img

all: $(ALL_IMGS)

define call_panda
	python3 ${VINTER_PATH}/target/release/vinter_trace.py --load-pmem ${1} --save-pmem $@ ${VINTER_PATH}/fs-testing/scripts/vm_winefs_aged.yaml --run
endef

.PHONY: clean
clean:
	rm -rf $(ALL_IMGS)

clean.img:
	truncate -s ${IMG_SIZE} clean.img

%.img: %.cmd clean.img
	$(call call_panda,clean.img) "echo `cat $< | base64 -w 0 -` | base64 -d | sh -"

repr_alloc_rec.img: repr_alloc_rec.cmd repr_alloc.img
	$(call call_panda,repr_alloc.img) "echo `cat $< | base64 -w 0 -` | base64 -d | sh -"

