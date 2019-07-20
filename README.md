# Krust [![Build Status](https://travis-ci.org/5mattmatt1/krust.svg?branch=basic_fat32)](https://travis-ci.org/5mattmatt1/krust)
This started out as an endeavor to better understand how software interacts with hardware and how linux kernel code works, learn some Rust programming along the way, and maybe even create a cool x86 kernel partially based off of Phillip Opperman's tutorials. Recently my goal has shifted to create my own Raspberry Pi kernel that has code that is a little more readable than what is in the linux kernel repository. 

# Peripherals
# UART0
This uart should be fully functional and uses the greatness of Rust macros to be able to 
have all the functionality of a printf function in C or the print macro in std rust.

# VidoeCore IV Mailbox
Please, please, please do not look at this code. I wrote that back in December and need to revamp how I was doing a lot of things to make myself not completely shamed of it
## Graphics
## UART1

# SDHCI
I based my interfacing with the SD card on Zoltan Baldaszti's code, but he was using the sdhci interface rather than the sdhost for talking to the SD card. This is definitely not what krust will have in the final version, as the only way to talk to the wireless interface for the Raspberry Pi 3/4/0W is through SDHCI, andd the SD card can be done much faster through the sdhost interace.

# SDHost (TODO)

# Filesystem
## Fat32 (WIP)
The general structures of fat32 have been implemented, but to really have anything worthwhile I need heap allocation and vectors

# Memory
## Dummy
Currently only have an allocator that causes a kernel panic when everything goes wrong. This will be in place until I can wrap my head around the ARM Cortex A53's Memory Management Unit and how linux uses the Slab memory allocation mechanism.

## Slab (TODO)
Linux uses a slab system for its memory allocation system, and I love the features and functionality of linux and plan to use Slab allocation.

# Interrupts/Syscalls (TODO)
Need to create a basic interrupt handler. Not sure, what all I would need them all for, but definitely want to add a svc handler and a syscall system.

# Userspace (TODO)
Want to create a basic userspace for krust so that I'm not always looking at crazy kernel code.

# Network (TODO)
Need to rework the SDHCI code to be able to interface with the wireless card and add an IP stack in.

# USB (TODO)
I'm not even thinking about the USB stack yet. Sadly, those new USB 3.0 ports on the Raspberry Pi 4 will be unused for the moment by krust.

# Futureeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeees

# Credits: 
I have had to draw from quite a few people's code bases for all of this work. I'll add hyperlinks for their awesome stuff later.

Andre Richter <andre.o.richter@gmail.com>
Phillip Opperman 
Zoltan Baldaszti (bztsrc@github)
David Welch
Peter Lemon
Leon De Boer