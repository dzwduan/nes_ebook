# 了解NES平台
## 架构

简化版的软硬件融合架构如下图所示：

<div style="text-align:center"><img src="./images/ch2/image_1_computer_arch.png" width="30%"/></div>

自顶向下来看:
* 应用运行业务逻辑并且通过操作系统与硬件交流。
* 操作系统通过机器语言沟通硬件。
* 在硬件层面，每个设备都可以被看作是一组存储元件、处理单元或两者兼而有之。从这个角度来看，NES手柄也不过是由8个1bit项组成的数组，每个bit代表一个按钮的按下或者释放。
* ALU和Memmory elements下面的层面不需要关心。在硬件层面上可以都归结为逻辑门的组合排列。

> 想要更多的了解软硬件的基础知识，推荐阅读《计算机系统要素》，非常具有实操性。

幸运的是，NES没有操作系统。这也意味着应用层(游戏内容)直接使用机器语言和硬件通信。

这种分层架构的简化版本如下图所示：

<div style="text-align:center"><img src="./images/ch2/image_2_nes_emul_arch.png" width="30%"/></div>

正如你所看到的，机器语言是应用层游戏和我们模拟器之间的接口。

在即将实现的模拟器中，我们需要实现上图中标绿的模块，`Computer Architecture`， `ALU` , `Memory elements`。通过使用高级语言，我们不需要考虑布尔算术和时序逻辑的模拟，反之，我们应该依赖现有的Rust特性和语言结构。

 

## NES平台主要组成部分

<div style="text-align:center"><img src="./images/ch2/image_3_nes_components.png" width="50%"/></div>

NES硬件组成的简化架构:

 * 中央处理单元(**CPU**) - NES的2A03芯片是6502芯片的修改版，用于执行主程序的命令。

* 图像处理单元 (**PPU**) - 基于2C02芯片，用于在电视屏幕上绘制游戏的当前状态。

* **CPU**和**PPU**都可以访问他们的2KiB(2048 bytes)的随机存储存储器(**RAM**)。

* 音频处理单元 (**APU**) - 该模块是2A03芯片的一部分，用于生成特定的基于五通道的声音，这使得NES芯片产生的声音具有辨识度。

* 卡带 - 是平台的重要组成部分，因为控制台没有操作系统。每个卡带至少有两个大的**ROM**芯片，字符ROM(CHR ROM)和程序ROM(PRG ROM)。前者存储游戏的视频图形数据，后者存储CPU指令--游戏代码。（当卡带插入插槽时，CHR ROM直连到 PPU，而 PRG ROM直连到 CPU)

  更高版本的卡带有额外的硬件(ROM和RAM)，可以通过映射器访问。这就解释了为什么运行在相同的控制台硬件上，后来的游戏却能够提供明显更好的游戏玩法和视觉效果。																		
  

<div style="text-align:center"><img src="./images/ch2/image_4_cartridge.png" width="50%"/></div>

* 游戏手柄 - 有一个明确的目标，即读取游戏玩家的输入并使其可用于游戏逻辑。我们稍后会看到，8-bit平台的游戏手柄只有8个按钮这一事实并非巧合。

CPU，PPU和APU彼此独立，这使得NES成为了一个分布式系统，其中单独的组件必须协调才能产生无缝的游戏体验。

我们可以使用主要的NES组件来实现我们的模拟器。

<div style="text-align:center"><img src="./images/ch2/image_6_impl_plan.png" width="80%"/></div>

我们必须实现所有相关模块的模拟。目标是快速开发，使用迭代开发方法，我们将逐步添加功能来实现目标。

粗略地估计一下每个组件的工作量，PPU是最难的，BUS是最简单的。

写一个完美的模拟器是是没有尽头的，我们将从模拟CPU开始。

<div style="text-align:center"><img src="./images/ch2/image_5_motherboard.png" width="80%"/></div>
