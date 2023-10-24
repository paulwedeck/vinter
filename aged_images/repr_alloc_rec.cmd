mount -t winefs /dev/pmem0 /mnt
mkdir /mnt/tmp || true
fallocate  -l 1 /mnt/abcd || true
rm /mnt/full_file
echo abcd >> /mnt/dummy || true
rm /mnt/dummy
touch /mnt/abcd
umount /mnt
