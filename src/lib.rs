//! DS3231 lib

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(never_type)]
#![no_std]
#![feature(reverse_bits)]

extern crate embedded_hal as hal;

// #[macro_use(block)]
// extern crate nb;


/// Device descriptor
#[derive(Clone, Copy, PartialEq)]
pub struct DS3231<S> {
    /// bob
    i2c : S,
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DS3231Regs {
    _Seconds = 0x0,
    _Minutes = 0x1,
    _Hours = 0x2,
    _Day = 0x3,
    _Date = 0x4,
    _MonthCentury = 0x5,
    _Year = 0x6,
    _Alrm1Secs = 0x7,
    _Alrm1Minutes = 0x8,
    _Alrm1Hours = 0x9,
    _Alrm1DayDate = 0xa,
    _Alrm2Minutes = 0xb,
    _Alrm2Hours = 0xc,
    _Alrm2DayDate = 0xd,
    _Control = 0xe,
    _ControlStatus = 0xf,
    _AgingOff = 0x10,
    TempMsb = 0x11,
    TempLsb = 0x12
}
const ADDR : u8 = 0x68u8;

impl<S> DS3231<S>
    where S: hal::blocking::i2c::Write + hal::blocking::i2c::WriteRead  {
    
    /// Creates a new device descriptor
    pub fn new(i2c: S) -> DS3231<S> {
        DS3231 {
            i2c : i2c
        }
    }

    ///bob
    pub fn get_temp(&mut self) -> (u8,u8) {
        let temp_msb_reg = [ DS3231Regs::TempMsb as u8];
        let temp_lsb_reg = [ DS3231Regs::TempLsb as u8];
        let mut temp_buf = [0];

        let val = match self.i2c.write_read(ADDR, &temp_msb_reg, &mut temp_buf) {
            Ok(_) => temp_buf[0] as u8,
            Err(_) => 0u8
        };
        match self.i2c.write_read(ADDR, &temp_lsb_reg, &mut temp_buf) {
            Ok(_) => (val, (temp_buf[0] >> 6) * 25),
            Err(_) => (val, 0)
        }
    }
}
