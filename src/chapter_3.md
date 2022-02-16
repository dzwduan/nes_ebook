 # 模拟CPU

本章的目标是让我们的第一个NES游戏启动并运行。
我们将会玩贪吃蛇游戏，[代码地址](https://gist.github.com/wkjagt/9043907)

 <div style="text-align:center"><img src="./images/ch3/snk_logo.png" width="40%"/></div>
 <div style="text-align:center"><img src="./images/ch3/snk_game.gif" width="40%"/></div>

CPU是任何计算机系统的心脏。CPU的工作是运行程序指令并且协调所有可用的硬件模块来提供完整的体验。尽管PPU和APU运行他们各自独立的电路，但是仍然需要按CPU节拍前进，并且执行CPU发出的命令。

在跳转到实现之前，我们需要简单地讨论一下CPU需要哪些资源来完成工作。

CPU唯二可用访问的资源是内存映射和CPU寄存器。

从编程的角度来看，内存映射只是一个1-bit单元的连续数组。NES CPU使用16位进行内存寻址，即可以寻址65536个不同的内存单元。

如我们之前所见，NES平台只有2KiB的RAM连接到CPU。

 <div style="text-align:center"><img src="./images/ch3/cpu_registers_memory.png" width="80%"/></div>


该RAM可以通过 **[0x0000 … 0x2000]**的地址空间访问。

访问 **[0x2000 … 0x4020]** 被重定向到其他可用的NES硬件模块: PPU, APU, GamePads等。

访问 **[0x4020 .. 0x6000]** 是特殊的，这里是不同的卡带所使用的特殊空间。该空间由所谓的映射器控制-卡带上的特殊电路。

Access to **[0x6000 .. 0x8000]** is reserved to a RAM space on a cartridge if a cartridge has one. It was used in games like Zelda for storing and retrieving the game state. We will ignore this space as well.

Access to **[0x8000 … 0x10000]** is mapped to Program ROM (PRG ROM) space on a cartridge.

Memory access is relatively slow, NES CPU has a few internal memory slots called registers with significantly lower access delay.


> | CPU Operation type  | Execution time (in CPU Cycles)  |
> |---|---|
> | Accessing only registers                         | 2        |
> | Accessing the first 255 bytes of RAM             | 3        |
> | Accessing memory space after the first 255         | 4-7  |


NES CPU has 7 Registers:
* Program Counter (*PC*) - holds the address for the next machine language instruction to be executed.
* Stack Pointer - Memory space [0x0100 .. 0x1FF] is used for stack. The stack pointer holds the address of the top of that space. NES Stack (as all stacks) grows from top to bottom: when a byte gets pushed to the stack, SP register decrements. When a byte is retrieved from the stack, SP register increments.

* Accumulator (*A*) - stores the results of arithmetic, logic, and memory access operations. It used as an input parameter for some operations.

* Index Register X (*X*) - used as an offset in specific memory addressing modes (more on this later). Can be used for auxiliary storage needs (holding temp values, being used as a counter, etc.)

* Index Register Y (*Y*) - similar use cases as register X.

* Processor status (*P*) - 8-bit register represents 7 status flags that can be set or unset depending on the result of the last executed instruction (for example Z flag is set (1) if the result of an operation is 0, and is unset/erased (0) otherwise)


Each CPU comes with a predefined hard-wired instruction set that defines everything a CPU can do.

CPU receives instructions from the application layer in the form of machine codes. And you can think of machine language as a thin layer connecting the software with the hardware.


Full lists of the official 6502 instructions:
* [http://www.obelisk.me.uk/6502/reference.html](http://www.obelisk.me.uk/6502/reference.html)
* [http://www.6502.org/tutorials/6502opcodes.html](http://www.6502.org/tutorials/6502opcodes.html)

I tend to use both of the links. The pages provide full specs of available CPU features and their machine codes.

I highly recommend reading this [interactive tutorial on 6502 instructions](https://skilldrick.github.io/easy6502/) before moving on.

 <div style="text-align:center"><img src="./images/ch3/image_4_opcodes.png" width="80%" /></div>

6502 chip is a relatively simple CPU; it supports only six types of commands and about 64 unique commands. Because some of the instructions have multiple versions for different memory addressing modes, it results in about 150 machine code operations that we are to implement.

> **NOTE:** NES console had a custom chip 2A03 that is based on 6502, but has noticeable differences:
>
> - in addition to official machine operations, it had about 110 unofficial additional opcodes (luckily, about a third of them are No-OPs)
> - it had Audio Processing Unit on-board
> - it didn't support decimal mode for arithmetic
>
> To keep things simple, we would need to implement support for 256 different machine instructions.
>
> The good news is that there are a lot of similarities between instructions. Once we have the foundation in place, we will be constantly reusing them to implement the whole set.
