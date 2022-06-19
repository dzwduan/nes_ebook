/*
 * @Date: 2022-06-19 22:20:51
 * @LastEditors: zhenwei duan
 * @LastEditTime: 2022-06-19 22:52:59
 * @FilePath: /nes_ebook/day1/src/nes6502.cpp
 */
#include "nes6502.h"



Nes6502::Nes6502() {

}

Nes6502::~Nes6502() {

}



void Nes6502::SetFlag(FLAGS f, bool v) {
    if (v)
        status |= f;
    else
        status &= ~f;
}

uint8_t Nes6502::GetFlag(FLAGS f) {
    return ((f & status) > 0 ? 1 : 0);
}