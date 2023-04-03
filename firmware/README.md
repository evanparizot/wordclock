# Overview


## Install Rust build target

```
rustup target add thumbv7em-none-eabihf
```

## Compiling
```
cargo build
```

## Start OpenOCD
```
openocd -s C:\OpenOCD\share\scripts -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

## Flashing Chip

OpenOCD in windows cmd
```
cd %TEMP%
openocd -s C:\share\scripts -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

Start Debug session
```
cargo run --target thumbv7em-none-eabihf
```

Connect to OpenOCD and connect to f3
```
(gdb) target remote :3333
```

Flash program
```
(gdb) load
```
## Hardware Debugging
```
# Need debug file for this to be useful
cargo run

# Adds breakpoint at main
(gdb) break main 

# Continue to next breakpoint
(gdb) continue

# Step one line
(gdb) step

# Shows inner code
(gdb) disassemble /m

# Print variable value
(gdb) print x
(gdb) print &x

# Print all local variables
(gdb) info locals

# Reset microcontroller
(gdb) monitor reset halt

# Set variable
(gdb) set variable = 100

```

## sout
Can print things back to the debugging computer through the use of `iprintln!()`

```
itmdump -F -f itm.txt
```

## Release
```
cargo build --target thumbv7em-none-eabihf --release

cargo size --target thumbv7em-none-eabihf --bin led-roulette -- -A
```