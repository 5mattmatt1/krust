# QEMU Testing
The problem with kernel development is that it is harder to automate Unit Testing due to the fact that the kernel is literally everything on the system. Certain future system level tests can be performed easier on the actual hardware and can actually be done using a simple UnitTesting through Rust or Python or any other system I choose. QEMU Testing will be focused on communicating between a qemu-system-aarch64 instance and a python script that will be handling the unit testing.