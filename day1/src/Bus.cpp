/*
 * @Date: 2022-06-19 12:25:41
 * @LastEditors: zhenwei duan
 * @LastEditTime: 2022-06-19 23:17:01
 * @FilePath: /nes_ebook/day1/src/Bus.cpp
 */
#include "Bus.h"

Bus::Bus()
{
    cpu.ConnectBus(this);
    for (auto &i : ram)
        i = 0x00;
}

uint8_t Bus::read(uint16_t addr, bool isReadOnly)
{
    if (addr >=0 && addr <= 0xFFFF)
        return ram[addr];
}

void Bus::write(uint16_t addr, uint8_t data) {
    if (addr >=0 && addr <= 0xFFFF)
        ram[addr] = data;

    return 0x00;
}