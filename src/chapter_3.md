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

访问 **[0x6000 .. 0x8000]**的权限被保留给卡带上的RAM空间（如果卡带有RAM空间的话）。它在塞尔达等游戏中用于存储和检索游戏状态。我们将会忽略这个空间。

访问 **[0x8000 … 0x10000]** 会被映射到卡带上的程序ROM(PRG ROM)。

内存访问相对较慢，NES CPU有内部存储器使得访问延迟显著降低。


> | CPU Operation type  | Execution time (in CPU Cycles)  |
> |---|---|
> | 只访问寄存器                   | 2        |
> | 访问RAM的前255bytes | 3        |
> | 访问前255bytes之后的内存空间 | 4-7  |


NES CPU有7个寄存器：
* 程序计数寄存器 (*PC*) - 保存下一条要执行的机器语言指令的地址。
* 栈指针寄存器(SP) - 内存空间 [0x0100 .. 0x1FF]用于栈。栈指针寄存器保存栈顶地址，NES的栈空间从上向下增长，当一个字节被压入栈时，SP寄存器递减，当一个字节从栈中弹出时，SP递增。

* 累加器(*A*) - 存储算术、逻辑和访问内存的操作结果，它被用于某些操作的输入参数。

* 索引寄存器X (*X*) - 用作特定寻址模式中的偏移量（稍后详细介绍），可用于辅助存储需求（存储temp值，用于计数等）。

* 索引寄存器 Y (*Y*) - 与X类似。

* 处理器状态寄存器 (*P*) - 8-bit寄存器代表了7个状态标志，可以根据最后执行指令的结果置位或者取消置位（例如：如果操作结果为0，则设置Z flag 为1，否则设置为0）。


每个CPU都带有一个预定义的硬接线的指令集，该指令集定义了CPU可以执行的所有操作。

CPU以机器码的形式从应用层接受指令，可以将机器语言看成是连接软件和硬件的接口。


官方6502指令的完整列表：
* [http://www.obelisk.me.uk/6502/reference.html](http://www.obelisk.me.uk/6502/reference.html)
* [http://www.6502.org/tutorials/6502opcodes.html](http://www.6502.org/tutorials/6502opcodes.html)

这些页面提供了可用的CPU功能和对应的机器码的完整规格。

我强烈推荐先阅读这个交互式教程 [interactive tutorial on 6502 instructions](https://skilldrick.github.io/easy6502/) 

 <div style="text-align:center"><img src="./images/ch3/image_4_opcodes.png" width="80%" /></div>

6502芯片是一个比较简单的CPU，仅仅支持6种类型的命令和64个独特的命令，因为有些指令对于不同的内存寻址有多个版本，这导致我们要实现大约150个机器码操作。

> **注意：** NES控制台有一个基于6502芯片的定制芯片2A03，但是有明显的差异：
>
> - 除官方机器操作码之外，还有110个非官方的附加操作码（其中大约1/3是空操作）
> - 它有板载音频处理单元
> - 它不支持算术的十进制模式
>
> 为了简单起见，我们需要对 256 种不同的机器指令提供支持。
>
> 好消息是指令之间有很多相似之处。 一旦我们有了基础，我们将不断地复用它们来实现完整的功能。
