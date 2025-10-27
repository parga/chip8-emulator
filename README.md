# Chip 8 emulation

 Introduction to emulation 

## What is it? 

CHIP-8 is an interpreted programming language, developed by Joseph Weisbecker on his 1802 microprocessor. It was initially used on the COSMAC VIP and Telmac 1800, which were 8-bit microcomputers made in the mid-1970s.


## What is it ?

Memory: 4kb 
Registers: V0 to VF
Stack: 16 slots { choose to have the stack in the allocated memory } 
Display buffer: 64x32 pixels


## Why ?

Is a great way to get your feet wet into OS emulation. You have to read binary from a rom, allocate memory, implement the chip 8 opcode spec and implement dispay and audio buffers 


## Implementation 

First you have to read the instructions, you need to read 2 bytes from the memory to have 1 instruction 


00000000 00000000

can be separated into 4 x 4 bits  aka nibles and each nible can be represented by 1 hex digit

1010 1100 0111 1111 = 0xAC7F


## Implementation 


I choose to use a bus for comunication between the memory , the cpu and the display 
