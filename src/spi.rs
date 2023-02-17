#![allow(dead_code)]

use jelly_mem_access::*;

const REG_SPI_STATUS: usize = 0x0;
const REG_SPI_CONTROL: usize = 0x1;
const REG_SPI_SEND: usize = 0x2;
const REG_SPI_RECV: usize = 0x3;
const REG_SPI_DIVIDER: usize = 0x4;
const REG_SPI_LSB_FIRST: usize = 0x5;

pub trait SpiAccess {
    fn set_cs_n(&self, cs_n: u8);
    fn exec(&self, data: u8) -> u8;
}

pub struct JellySpi<T: MemAccess> {
    reg_acc: T,
    wait_irq: Option<fn()>,
}

impl<T: MemAccess> JellySpi<T> {
    pub const fn new(reg_acc: T, wait_irq: Option<fn()>) -> Self {
        Self {
            reg_acc: reg_acc,
            wait_irq: wait_irq,
        }
    }

    fn wait(&self) {
        while (unsafe { self.reg_acc.read_reg(REG_SPI_STATUS) } & 1) != 0 {
            if let Some(wait_fn) = self.wait_irq {
                wait_fn();
            }
        }
    }

    pub fn set_divider(&self, div: u16) {
        unsafe {
            self.reg_acc.write_reg16(REG_SPI_DIVIDER, div);
        }
    }
}

impl<T: MemAccess> SpiAccess for JellySpi<T> {
    fn set_cs_n(&self, cs_n: u8) {
        unsafe {
            self.reg_acc.write_reg8(REG_SPI_CONTROL, cs_n);
        }
    }

    fn exec(&self, data: u8) -> u8 {
        self.wait();
        unsafe {
            self.reg_acc.write_reg8(REG_SPI_SEND, data);
        }
        self.wait();
        unsafe { self.reg_acc.read_reg8(REG_SPI_RECV) }
    }
}
