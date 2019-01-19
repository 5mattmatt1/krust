# Krust
Krust is an open source kernel written in Rust primarily for the arm architecture (specifically the Broadcom 2855 SOC found within the  Raspberry Pi). The goal of this kernel is mostly to be a learning exercise to better understand how a systems programming language interacts with the hardware, and how the linux kernel functions as I will be drawing heavily from the linux kernel codebase on understanding problems and put more emphasis on figuring out WHY those solutions work. 

# Goals and TODO List
- [X] A working linker script and Makefile that can continously be used.
- [X] Interaction with hardware peripherals.
- [X] Working GPIO pins including STATUS LED.
- [X] Logging over the TX and RX GPIO Pins. (Currently only useful on QEMU-ARM for me.)
- [ ] **Functioning GPU Framebuffer**
>- [X] Able to interact with the GPU mailbox.
>- [ ] Ability to remap memory in order to interact with the GPU mailbox.
>- [ ] *Possibly other things needed...*
- [ ] A functioning USB stack, based on [CSUD](https://github.com/Chadderz121/csud).
>- [ ] A working keyboard using aforementioned USB stack.
>- [ ] A working mouse using aforementioned USB stack. (Low priority, since mice are mostly for GUIs). 
>- [ ] Extending the CSUD driver to interact with the USB Ethernet hub on the Raspberry Pi.
- [ ] Creating a simple ASCII console that combines the GPU framebuffer and the USB keyboard to create a simple input/output kernel. (First milestone)
- [ ] Building off the USB Ethernet Hub to create a network stack.
>- [ ] Implementing the TCP protocol.
>>- [ ] Creating the TCP handshake (SYNACK) and simple socket functionality. 
>>- [ ] *Creating system calls for these sockets so that there is an effective userspace!!!*

# Important links for Research
[Baking Pi – Operating Systems Development](https://www.cl.cam.ac.uk/projects/raspberrypi/tutorials/os/)
*Known issues*:
* Starting Peripheral address is not compatible with Raspberry Pi 2 & 3 (Use 0x3F00_0000 instead of 0x2000_0000).
* Raspberry Pi 2 & 3 Uses channel 8 (the property channel) for allocating framebuffers instead of channel 1.

<!-- (End of bulleted list) -->
[The Mailbox Peripheral](https://jsandler18.github.io/extra/mailbox.html)

[The Property Mailbox Channel](https://jsandler18.github.io/extra/prop-channel.html)

[Writing an OS in Rust 2nd Edition](https://os.phil-opp.com/) (*Note is used for a x86 architechture not an ARM based one*)
