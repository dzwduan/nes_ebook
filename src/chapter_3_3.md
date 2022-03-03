# Implementing the rest of CPU instructions


 <div style="text-align:center"><img src="./images/ch3.3/image_1_how_to_draw_owl.png" width="60%"/></div>



实现剩下的6502指令应该相当简单，这里不花费篇幅介绍。

有一些要注意的地方:

* ADC指令可能是逻辑上最复杂的指令。包含可以跳过十进制的相关信息，需要阅读文档。
> 这篇文章详细介绍了如何在6502中实现二进制算术: [The 6502 overflow flag explained mathematically ](http://www.righto.com/2012/12/the-6502-overflow-flag-explained.html)
>
>如果还比较好奇，看这里: [The 6502 CPU's overflow flag explained at the silicon level ](http://www.righto.com/2013/01/a-small-part-of-6502-chip-explained.html)

* 实现了ADC之后，SBC也能相应实现出来
`A - B = A + (-B)`.
And `-B = !B + 1`

* **PHP**, **PLP** 和 **RTI** 必须处理 [2 bit B-flag](http://wiki.nesdev.com/w/index.php/Status_flags#The_B_flag). 除了中断执行, 只有这些指令能够直接或者被直接影响Sstatus寄存器P的第五个bit

* 大部分的branch和jmp操作都可以简单的通过修改pc来实现. 但是注意不要在同一指令周期内递增寄存器

如果你卡住了，可以在这里找6502指令集的实现: <link to code>

<br/>

------

> The full source code for this chapter: <a href="https://github.com/bugzmanov/nes_ebook/tree/master/code/ch3.3" target="_blank">GitHub</a>

