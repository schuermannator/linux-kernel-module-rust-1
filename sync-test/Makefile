KDIR ?= /lib/modules/$(shell uname -r)/build

KBFLAGS := CC:=clang

.PHONY: all debug release clean

all:
	$(MAKE) -C $(KDIR) M=$(CURDIR) $(KBFLAGS)

debug: all

release: KBFLAGS += CARRELEASE:=1
release: all

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) clean
