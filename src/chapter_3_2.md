#  内存映射模型

在我们原始的实现中，CPU接受指令序列作为单独的输入流，但这不是NES的实际工作方式。

NES实现了典型的冯诺依曼架构：数据和指令都存储在内存中。从CPU的角度来看，执行的代码也是数据，任何数据都能被解释为可执行的代码，CPU无法区分二者，CPU只能使用PC寄存器来跟踪当前在指令流中的位置。

 <div style="text-align:center"><img src="./images/ch3.2/image_1_von_neuman.png" width="60%"/></div>

修改代码如下：

```rust

pub struct CPU {
   pub register_a: u8,
   pub register_x: u8,
   pub status: u8,
   pub program_counter: u16,
   memory: [u8; 0xFFFF]
}

impl CPU {

    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.run()
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        self.program_counter = 0x8000;
    }

    pub fn run(&mut self) {
    // note: we move  intialization of program_counter from here to load function
        loop {
            let opscode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            match opscode {
                //..
            }
        }
    }
}

```


我们为64KiB的地址空间创建了一个数组，但是CPU只有2KiB的RAM，其他部分都作为保留部分。

我们从0x8000开始将程序加载到内存中，[0x8000...0xffff]之前提到过，是为程序ROM保留的区域，假设指令流从这块空间的某个地址开始，不一定是0x8000。

NES平台有一个特殊的机制来标记CPU从哪里开始执行。插入卡带后，CPU会收到一个称为重置中断的特殊信号，该信号导致CPU做出以下行为：
* 重置状态（寄存器和flags)
* 将PC设置为存储在0xFFFC的16位地址

在实现之前，回顾之前的内容提到NES CPU可以寻址65536个内存单元。存储一个地址需要两个字节，NES采用小端寻址。

为了说明差异：


| |  |
|-|-:|
| 真实地址 | **0x8000** |
| 大端模式 | **80 00** |
|小端模式 | **00 80** |


例如，将数据从内存单元0x8000读取到A寄存器的指令如下所示，这里0080是传入的顺序，而$8000是我们想要的顺序，因此写入ad 00 80 先写低位，再写高位

```
LDA $8000      <=>    ad 00 80
```

我们可以使用rust的位运算来实现该行为：


```rust
   fn mem_read_u16(&mut self, pos: u16) -> u16 {
       let lo = self.mem_read(pos) as u16;
       let hi = self.mem_read(pos + 1) as u16;
       (hi << 8) | (lo as u16)
   }

   fn mem_write_u16(&mut self, pos: u16, data: u16) {
       let hi = (data >> 8) as u8;
       let lo = (data & 0xff) as u8;
       self.mem_write(pos, lo);
       self.mem_write(pos + 1, hi);
   }

```

或者使用rust实现好的 [endian support for primitive types](https://doc.rust-lang.org/std/primitive.u16.html#method.from_le_bytes).

现在我们已经能够实现reset功能了，需要调整`load` 和 `load_and_run`函数：
* **load** 方法将程序加载到程序ROM空间并且在0xFFFC处保存对代码的引用。
* **reset** 方法恢复所有寄存器的状态，通过存储在0xFFFC处的两个字节初始化PC


```rust
   pub fn reset(&mut self) {
       self.register_a = 0;
       self.register_x = 0;
       self.status = 0;

       self.program_counter = self.mem_read_u16(0xFFFC);
   }

   pub fn load(&mut self, program: Vec<u8>) {
       self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
       self.mem_write_u16(0xFFFC, 0x8000);
   }

   pub fn load_and_run(&mut self, program: Vec<u8>) {
       self.load(program);
       self.reset();
       self.run()
   }

```

现在需要修改测试例：

上一节的LDA实际上可以根据参数类型分为8种不同的机器指令


 <div style="text-align:center"><img src="./images/ch3.2/image_2_opcodes.png" width="80%"/></div>

你可以在下面阅读寻址模式相关内容
- [here](https://skilldrick.github.io/easy6502/#addressing)
- and [here](https://www.obelisk.me.uk/6502/addressing.html)

简而言之，寻址模式是指令的一个属性，它定义了 CPU 应该如何解释指令流中接下来的 1 或 2 个字节。

不同的寻址模式有不同的指令大小，例如：
- **Zero Page version** ($A5) 有两个字节，一个是opcode，一个是参数。这就是为什么zero page寻址不能引用前 255 个字节以上的内存。
- **Absolute version** ($AD) 有3字节, 地址占用两个字节从而可以引用65536个内存单元。(*注意*：地址占用使用小端模式)

没有超过3字节的opcode。CPU指令只有1，2或者3字节。

大多数CPU提供了不止一种寻址方式，我们也不想为每条 CPU 指令重新实现相同的寻址模式。


下面尝试编写CPU如何解释不同的寻址模式：

```rust

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
   Immediate,
   ZeroPage,
   ZeroPage_X,
   ZeroPage_Y,
   Absolute,
   Absolute_X,
   Absolute_Y,
   Indirect_X,
   Indirect_Y,
   NoneAddressing,
}

impl CPU {
   // ...
   fn get_operand_address(&self, mode: &AddressingMode) -> u16 {

       match mode {
           AddressingMode::Immediate => self.program_counter,

           AddressingMode::ZeroPage  => self.mem_read(self.program_counter) as u16,
          
           AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
        
           AddressingMode::ZeroPage_X => {
               let pos = self.mem_read(self.program_counter);
               let addr = pos.wrapping_add(self.register_x) as u16;
               addr
           }
           AddressingMode::ZeroPage_Y => {
               let pos = self.mem_read(self.program_counter);
               let addr = pos.wrapping_add(self.register_y) as u16;
               addr
           }

           AddressingMode::Absolute_X => {
               let base = self.mem_read_u16(self.program_counter);
               let addr = base.wrapping_add(self.register_x as u16);
               addr
           }
           AddressingMode::Absolute_Y => {
               let base = self.mem_read_u16(self.program_counter);
               let addr = base.wrapping_add(self.register_y as u16);
               addr
           }

           AddressingMode::Indirect_X => {
               let base = self.mem_read(self.program_counter);

               let ptr: u8 = (base as u8).wrapping_add(self.register_x);
               let lo = self.mem_read(ptr as u16);
               let hi = self.mem_read(ptr.wrapping_add(1) as u16);
               (hi as u16) << 8 | (lo as u16)
           }
           AddressingMode::Indirect_Y => {
               let base = self.mem_read(self.program_counter);

               let lo = self.mem_read(base as u16);
               let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
               let deref_base = (hi as u16) << 8 | (lo as u16);
               let deref = deref_base.wrapping_add(self.register_y as u16);
               deref
           }
         
           AddressingMode::NoneAddressing => {
               panic!("mode {:?} is not supported", mode);
           }
       }

   }

```

我们初始的LDA实现也需要更改：

```rust
 fn lda(&mut self, mode: &AddressingMode) {
       let addr = self.get_operand_address(mode);
       let value = self.mem_read(addr);
      
       self.register_a = value;
       self.update_zero_and_negative_flags(self.register_a);
   }


   pub fn run(&mut self) {
       loop {
           let code = self.mem_read(self.program_counter);
           self.program_counter += 1;

           match code {
               0xA9 => {
                   self.lda(&AddressingMode::Immediate);
                   self.program_counter += 1;
               }
               0xA5 => {
                   self.lda(&AddressingMode::ZeroPage);
                   self.program_counter += 1;
               }
               0xAD => {
                   self.lda(&AddressingMode::Absolute);
                   self.program_counter += 2; 
               }
               //....
           }
       }
   }

```


NOTE: It's absolutely necessary to increment **program_counter** after each byte being read from the instructions stream.

不要忘记测试

```rust
   #[test]
   fn test_lda_from_memory() {
       let mut cpu = CPU::new();
       cpu.mem_write(0x10, 0x55);

       cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

       assert_eq!(cpu.register_a, 0x55);
   }
```

使用同样的方法，我们能快速实现STA指令，它将值从寄存器复制到内存


```rust
    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }


    pub fn run(&mut self) {
//...
        match code {
            //..
            /* STA */
            0x85 => {
                self.sta(AddressingMode::ZeroPage);
                self.program_counter += 1;
            }

            0x95 => {
                self.sta(AddressingMode::ZeroPage_X);
                self.program_counter += 1;
            }
            //..
        }
    }
```

在结束之前，我想说明一下现在的run方法因为一些原因不太适用：

1.在一些操作后pc加1和加2很容易出错并且难以检测到。

2.如果将所有LDA操作归到同一个match语句下是不是更具有可读性和方便性？

3.我们所作的工作主要是将指令规范用rust实现，翻译比较困难，以表格的形式管理代码是否会容易一点？（见下图）

 <div style="text-align:center"><img src="./images/ch3.2/image_3_ideal_state.png" width="80%"/></div>

如何实现留给读者思考。

<br/>

------

> The full source code for this chapter: <a href="https://github.com/bugzmanov/nes_ebook/tree/master/code/ch3.2" target="_blank">GitHub</a>




