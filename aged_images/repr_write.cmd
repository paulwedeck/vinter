mount -t winefs -o init,strict /dev/pmem0 /mnt
fallocate -o 4096 -l 100000000 /mnt/full_file || true
dd if=/dev/zero of=/mnt/file bs=1 count=1
#dmesg
umount /mnt

