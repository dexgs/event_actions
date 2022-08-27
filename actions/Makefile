#!/usr/bin/env make
CFLAGS += -Wall -Wextra
CFLAGS += -Iinclude
-include .makerc

csrc := $(shell find src -name "*.c")
obj := $(csrc:%.c=obj/%.o)
obj/%.o: %.c
	@mkdir -p "$(@D)"
	@$(CC) -c $(CFLAGS) $< -o $@

lib/obj.so: $(obj)
	@mkdir -p lib
	$(CC) -shared $(CFLAGS) $^ -o $@ $(LDFLAGS)

.PHONY: clean
clean:
	$(RM) -r obj
	$(RM) -r lib
