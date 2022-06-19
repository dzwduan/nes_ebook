/*
 * @Date: 2022-06-19 12:25:41
 * @LastEditors: zhenwei duan
 * @LastEditTime: 2022-06-19 22:05:42
 * @FilePath: /nes_ebook/day1/src/Bus.cpp
 */
#include "Bus.h"

Bus::Bus()
{
    for (auto &i : ram)
        i = 0x00;
}

uint8_t read(uint16_t addr, bool isReadOnly)
{
    if (addr >=0 && addr <= 0xFFFF)
        return ram[addr];
}

void write(uint16_t addr, uint8_t data) {
    if (addr >=0 && addr <= 0xFFFF)
        ram[addr] = data;
}