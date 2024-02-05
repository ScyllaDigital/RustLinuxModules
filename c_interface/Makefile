# Run this Makefile from the c_interface directory
obj-m    := crs.o
crs-objs := c_interface.o librs_interface.a

# distro := "$(shell read -r first_token _rest < /etc/issue; echo "$$first_token")"
kernel_headers = "/lib/modules/$(shell uname -r)/build"


all: c_interface.c librs_interface.a
	make -C $(kernel_headers) M=$(PWD) modules

# c_interface.o: librs_interface.a
# 	cc -static c_interface.c librs_interface.a

librs_interface.a:
	cargo build -p rs-interface --release
	cp ../target/release/librs_interface.a ./librs_interface.a


clean:
	make -C $(kernel_headers) M=$(PWD) clean