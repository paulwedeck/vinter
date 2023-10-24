mount -t winefs -o init,strict /dev/pmem0 /mnt
fallocate -l 1 /mnt/dummy
fallocate -o 4096 -l 100000000 /mnt/full_file || true
rm /mnt/dummy
umount /mnt

