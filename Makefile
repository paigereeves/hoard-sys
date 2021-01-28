include $(out)/GNUmakefile

CPPFLAGS=-std=c++14 -flto -O3 -DNDEBUG -ffast-math -fno-builtin-malloc -Wall -Wextra -Wshadow -Wconversion -Wuninitialized -Dalways_inline= 

Hoard:
	cd $(out) && $(MAKE) $(shell uname -s)-gcc-$(shell uname -p)-static CPPFLAGS="$(CPPFLAGS)"
