/*
 * @Date: 2022-06-19 03:07:50
 * @LastEditors: zhenwei duan
 * @LastEditTime: 2022-06-19 22:01:45
 * @FilePath: /nes_ebook/day1/include/Bus.h
 */
#include <cstdint>
#include "nes6502.h"
#include <array>


class Bus {
    public:
        Bus();
        ~Bus();

    public:
        Nes6502 cpu;
        std::array<uint8_t, 64*1024> ram;

        uint8_t read(uint16_t addr, bool isReadOnly);
        void write(uint16_t addr, uint8_t data);
};