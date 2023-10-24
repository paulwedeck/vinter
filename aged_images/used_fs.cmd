mount -t winefs -o init,strict /dev/pmem0 /mnt
fallocate -l 1 /mnt/dummy1
fallocate -l 1 /mnt/dummy2
fallocate -l 5000000 /mnt/full_file
mkdir /mnt/dummy3
fallocate -o 4096 -l 100000000 /mnt/full_file2 || true
rm /mnt/dummy1 /mnt/dummy2
rmdir /mnt/dummy3
umount /mnt

