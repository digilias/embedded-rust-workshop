all:
	arm-none-eabi-gcc \
	  -c src/lowpass_filter.c \
	  -o lib/lowpass_filter.o \
	  -I include \
	  -mthumb \
	  -march=armv8-m.main \
	  -mfloat-abi=hard \
	  -mfpu=fpv5-sp-d16 \
	  -ffunction-sections \
	  -fdata-sections \
	  -O2 \
	  -std=c11 \
	  -Wall -Wextra
	arm-none-eabi-ar rcs lib/liblowpass_filter.a lib/lowpass_filter.o
