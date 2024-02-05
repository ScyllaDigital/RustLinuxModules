Be aware this is a prototype.
Be careful, although this should only do a kernel message, it is still a kernel module.

The actual place to use make is in c_interface, where a makefile resides.
it allows use of `make` and `make clean`.

intened behavior is 
```bash
  cd c_interface
  make
  insmod crs.ko
  rmmod crs.ko
  dmesg | tail
```

and a greeting should be visible in the last two kernel logs

it currently does not allow `insmod` to work on my machine 