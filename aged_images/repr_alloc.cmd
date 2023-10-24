mount -t winefs -o init,strict /dev/pmem0 /mnt
#fallocate -l 1 /mnt/dummy
#touch /mnt/dummy
fallocate  -l 100000000 /mnt/full_file || true
#rm /mnt/dummy
fallocate -l 4097 /mnt/dummy
umount /mnt

