# Let's get started.

 <div style="text-align:center"><img src="./images/ch3/chapter_logo.png" width="20%"/></div>

首先尝试解释第一个程序，该程序如下所示：


```
a9 c0 aa e8 00
```

该二进制代码对人类阅读不友好。如果我们用汇编表示程序，可以更清除地看到发生了什么：

<div style="text-align:center"><img src="./images/ch3.1/image_1_assembler.png" width="40%"/></div>

现在更具可读性：由4条指令组成，第一条指令有一个参数

6502指令集参考 ：

https://skilldrick.github.io/easy6502/

https://www.masswerk.at/6502/6502_instruction_set.html

http://www.c64os.com/post/6502instructions

<div style="text-align:center"><img src="./images/ch3.1/image_2_lda_spec.png" width="50%"/></div>

该命令将0xc0加载到累加寄存器A中，还需要更新状态寄存器P的一些位（bit 1 - Zero Flag和bit 7 - Negative Flag）


> LDA规范显示0xA9有一个参数，指令大小为2字节，一个用于操作码本身，一个用于参数。
>
> NES操作码有0个或1个显式参数，对于某些操作，显式参数可以占用两个字节，此时机器指令将占用3个字节。
>
> 一些操作使用PC作为隐式参数

让我们俯瞰一下CPU的结构：

```rust
pub struct CPU {
   pub register_a: u8,
   pub status: u8,
   pub program_counter: u16,
}
 
impl CPU {
   pub fn new() -> Self {
       CPU {
           register_a: 0,
           status: 0,
           program_counter: 0,
       }
   }
 
   pub fn interpret(&mut self, program: Vec<u8>) {
       todo!("")
   }
}
```

我们引入了程序计数器PC，用于追踪程序的当前位置。interpret采用可变引用，因为需要在执行期间修改`register_a`

CPU工作中以恒定的周期工作:
* 从指令存储器中取出下一条指令
* 译码
* 执行指令
* 重复循环

我们尝试实现todo

```rust 
pub fn interpret(&mut self, program: Vec<u8>) {
    self.program_counter = 0;

    loop {
        let opscode = program[self.program_counter as usize];
        self.program_counter += 1;

        match opscode {
            _ => todo!()
        }
    }
}
```

到目前位置一切正常，循环的退出后面实现，下面实现LDA(0xA9)

```rust
        match opscode {
            0xA9 => {
                let param = program[self.program_counter as usize];
                self.program_counter +=1;
                self.register_a = param;

                if self.register_a == 0 {
                    self.status = self.status | 0b0000_0010;
                } else {
                    self.status = self.status & 0b1111_1101;
                }

                if self.register_a & 0b1000_0000 != 0 {
                    self.status = self.status | 0b1000_0000;
                } else {
                    self.status = self.status & 0b0111_1111;
                }

            }
            _ => todo!()
        }
```

我们并没有做一些神奇的事情，仅仅是按照手册使用rust构建二进制算术功能。

> 根据结果对CPU的flag置位或者取消置位比较重要。

由于无限循环，我们还不能测试功能，这里需要实现BRK(0x00)：

```rust
        match opcode {
        // ...
            0x00 => {
                return;
            }
            _ => todo!()
        }
```

现在写一些测试：


```rust
#[cfg(test)]
mod test {
   use super::*;
 
   #[test]
   fn test_0xa9_lda_immidiate_load_data() {
       let mut cpu = CPU::new();
       cpu.interpret(vec![0xa9, 0x05, 0x00]);
       assert_eq!(cpu.register_a, 0x05);
       assert!(cpu.status & 0b0000_0010 == 0b00);
       assert!(cpu.status & 0b1000_0000 == 0);
   }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }
}
```

> 这些已经足够了吗？ 还需要检查哪些内容？

现在我们来实现另一个操作码TAX

<div style="text-align:center"><img src="./images/ch3.1/image_3_tax_spec.png" width="50%"/></div>

功能比较简单：将值从A寄存器复制到X寄存器，并更新状态寄存器。

我们需要在CPU中引入`register_x`，然后就能实现TAX(0xAA)：

```rust
pub struct CPU {
//...
   pub register_x: u8,
}

impl CPU {
// ...    
    pub fn interpret(&mut self, program: Vec<u8>) {
// ...
        match opscode {
            //...  
            0xAA =>  {
                self.register_x = self.register_a;
            
                if self.register_x == 0 {
                    self.status = self.status | 0b0000_0010;
                } else {
                    self.status = self.status & 0b1111_1101;
                }

                if self.register_x & 0b1000_0000 != 0 {
                    self.status = self.status | 0b1000_0000;
                } else {
                    self.status = self.status & 0b0111_1111;
                }

            }
        }
    }
}
```

不要忘记写测试：


```rust 
   #[test]
   fn test_0xaa_tax_move_a_to_x() {
       let mut cpu = CPU::new();
       cpu.register_a = 10;
       cpu.interpret(vec![0xaa, 0x00]);
 
       assert_eq!(cpu.register_x, 10)
   }
```

在转到实现下一个opcode之前，我们必须承认现在的代码比较复杂
* interpret方法较为复杂，实现的功能太多
* TAX和LDA的实现方式有重复

重新修改如下：

```rust 
// ... 
  fn lda(&mut self, value: u8) {
       self.register_a = value;
       self.update_zero_and_negative_flags(self.register_a);
   }
 
   fn tax(&mut self) {
       self.register_x = self.register_a;
       self.update_zero_and_negative_flags(self.register_x);
   }
  
    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }
// ...    
    pub fn interpret(&mut self, program: Vec<u8>) {
// ...
        match opscode {
            0xA9 => {
                let param = program[self.program_counter as usize];
                self.program_counter += 1;
                
                self.lda(param);
            }

            0xAA => self.tax(),

            0x00 => return,
            
            _ => todo!(),
        }
    }
}
```

现在的代码看起来更容易管理，跑一下测试吧。

为所有的opcode编写对应的测试非常重要，微小的错误就可能导致游戏逻辑中预科预知的影响。

<div style="text-align:center"><img src="./images/ch3.1/image_4_pacman_bug.gif" width="30%"/></div>

实现程序中的最后一个opcode也不是难事，作为exercise留下。

实现完成之后进程下面的测试：

```rust 
   #[test]
   fn test_5_ops_working_together() {
       let mut cpu = CPU::new();
       cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
 
       assert_eq!(cpu.register_x, 0xc1)
   }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }
```
<br/>

------

> The full source code for this chapter: <a href="https://github.com/bugzmanov/nes_ebook/tree/master/code/ch3.1" target="_blank">GitHub</a>.
