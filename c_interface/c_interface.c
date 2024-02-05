#include <linux/module.h>
#include <linux/init.h>

/* Meta Information */
MODULE_LICENSE("None");
MODULE_AUTHOR("Remy Pierre Bushnell Clarke");
MODULE_DESCRIPTION("A hello world LKM");

extern int __rust_init(void);
extern void __rust_exit(void);

void printk_(char* fmt, ...) { 
  va_list ptr;
  printk(fmt,ptr);
};



/**
 * @brief This function is called, when the module is loaded into the kernel
 */
static int __init ModuleInit(void) {
  // printk("Hello, kernel!\n");
  // return 0;

  return __rust_init();
}

/**
 * @brief This function is called, when the module is removed from the kernel
 */
static void __exit ModuleExit(void) {
  // printk("Goodbye, Kernel\n");

  __rust_exit();
}

module_init(ModuleInit);
module_exit(ModuleExit);
