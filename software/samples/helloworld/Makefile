target ?= thumbv7m-none-eabi


all: build asm bin

build:
	@cargo build --target $(target)

asm:
	@arm-none-eabi-objdump -DSla target/$(target)/debug/helloworld > target/helloworld.asm
bin:
	@#cargo objcopy --target $(target) --bin helloworld -- -O binary helloworld.bin


run:
	@qemu-system-arm -cpu cortex-m3 -machine lm3s6965evb -nographic -semihosting -kernel target/$(target)/debug/helloworld

clean:
	cargo clean
	rm -f *.bin *.asm *.map