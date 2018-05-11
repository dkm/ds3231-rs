//! DS3231 lib

#![deny(missing_docs)]
#![deny(warnings)]
#![feature(never_type)]
#![no_std]
#![feature(reverse_bits)]

extern crate embedded_hal as hal;
extern crate numtoa;

use numtoa::NumToA;
// #[macro_use(block)]
// extern crate nb;

/// Device descriptor
#[derive(Clone, Copy, PartialEq)]
pub struct DS3231<S> {
    /// bob
    i2c : S,
}

/// Time struct
#[derive(Clone, Copy, PartialEq, Default)]
pub struct DS3231Time {
    secs : u8,
    mins : u8,
    hours : u8,
    wday : u8,
    mday : u8,
    month : u8,
    year : u8
}

impl DS3231Time {
    /// Returns simple string
    pub fn get_simple(&self, buf : &mut [u8]) {
        let mut digits = [0u8;4];

        let s = self.mins.numtoa(10, &mut digits);
        if s == 3 {
            buf[1] = digits[s];
        } else {
            buf[0] = digits[s];
            buf[1] = digits[s+1];
        }

        let s = self.secs.numtoa(10, &mut digits);
        if s == 3 {
            buf[3] = digits[s];
        } else {
            buf[2] = digits[s];
            buf[3] = digits[s+1];
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DS3231Regs {
    Seconds = 0x0,
    Minutes = 0x1,
    Hours = 0x2,
    Day = 0x3,
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

    fn get_reg(&mut self, addr : DS3231Regs) -> u8 {
        let addr_reg = [ addr as u8];
        let mut buf = [0];

        match self.i2c.write_read(ADDR, &addr_reg, &mut buf) {
            Ok(_) => buf[0] as u8,
            Err(_) => 0u8
        }
    }

    fn set_reg(&mut self, addr : DS3231Regs, val: u8) {
        let write_cmd = [ addr as u8, val];

        // match i2c.write(ds3231_adr, &write_seconds) {
        match self.i2c.write(ADDR, &write_cmd) {
            Ok(_) => (),
            Err(_) => (),
        }
    }


    /// Returns the temperature as a tuple of (integer part, decimal part).
    /// Decimal part begin one of 0,25 and 75.
    pub fn get_temp(&mut self) -> (u8,u8) {
        // let temp_msb_reg = [ DS3231Regs::TempMsb as u8];
        // let temp_lsb_reg = [ DS3231Regs::TempLsb as u8];
        // let mut temp_buf = [0];

        (self.get_reg(DS3231Regs::TempMsb), self.get_reg(DS3231Regs::TempLsb))
        // let val = match self.i2c.write_read(ADDR, &temp_msb_reg, &mut temp_buf) {
        //     Ok(_) => temp_buf[0] as u8,
        //     Err(_) => 0u8
        // };
        // match self.i2c.write_read(ADDR, &temp_lsb_reg, &mut temp_buf) {
        //     Ok(_) => (val, (temp_buf[0] >> 6) * 25),
        //     Err(_) => (val, 0)
        // }
    }

    /// Returns the temperature as a float.
    pub fn get_temp_float(&mut self) -> f32 {
        let (int, dec) = self.get_temp();
        int as f32 + dec as f32 / 100f32
    }

    /// Sets the time
    pub fn set_time(&mut self, time : &DS3231Time) {
        self.set_reg(DS3231Regs::Seconds, time.secs);
        self.set_reg(DS3231Regs::Minutes, time.mins);
        self.set_reg(DS3231Regs::Hours, time.hours);
        self.set_reg(DS3231Regs::Day, time.wday);
    }

    /// Returns the time from device
    pub fn get_time(&mut self) -> DS3231Time {
        let mut ret_time : DS3231Time = Default::default();

        let secs_val = self.get_reg(DS3231Regs::Seconds);
        ret_time.secs = (secs_val & 0xF) + (10 * (secs_val>>4));

        let mins_val = self.get_reg(DS3231Regs::Minutes);
        ret_time.mins = (mins_val & 0xF) + (10 * (mins_val>>4));

        let hours_val = self.get_reg(DS3231Regs::Hours);
        ret_time.hours = (hours_val & 0xF) + (10 * (hours_val>>4));

        ret_time.wday = self.get_reg(DS3231Regs::Day);

        ret_time
    }
}
