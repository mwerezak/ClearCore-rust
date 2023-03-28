# Program/Debug the ClearCore using Atmel-ICE and OpenOCD

## Before getting started
1. Make sure to get the right cable - TC2030-CTX-LEMTA

   The regular TC2020 (non-LEMTA version) has a different pin map and won't work.

2. Connect the JTAG end of the cable to thge SAM connector on the Atmel-ICE

## Commands

### Reset the ClearCore
```
openocd -f clearcore.cfg -c "init;reset;shutdown"
```

### Program the ClearCore
```
openocd -f clearcore.cfg -c "program {path to .bin file} verify reset 0x4000;shutdown"
```

### Start Debug Session
```
openocd -f clearcore-debug.cfg
```

## Debugging
Once the debug session has started, note the port number that it is listening on and launch `arm-none-eabi-gdb`, then issue the commands:
```
target remote localhost:{port}
break {func}
monitor reset halt
continue
```
