#include <linux/cdev.h>
#include <linux/fs.h>
#include <linux/module.h>
#include <linux/random.h>
#include <linux/slab.h>
#include <linux/uaccess.h>
#include <linux/version.h>
#include <linux/netdevice.h>
#include <net/ip_tunnels.h>
#include <linux/skbuff.h>
#include <linux/inet.h>
#include <linux/ip.h>
#include <linux/inetdevice.h>

// Custom additions for locks
#include <linux/spinlock.h>
#include <linux/spinlock_types.h>
#include <linux/mutex.h>


// Bindgen gets confused at certain things
//
const gfp_t BINDINGS_GFP_KERNEL = GFP_KERNEL;
