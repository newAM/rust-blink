#![no_std]
#![no_main]

// Halt when the program panics.
extern crate cortex_m_rt;
extern crate mkl25z4;
extern crate panic_semihosting;

// Includes.
use cortex_m_rt::entry;
use mkl25z4::Interrupt;
//use cortex_m_rt::interrupt;
//use cortex_m_rt;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    // Check out the 'Cortex-M Peripherals' singleton.
    //let _cm_p = cortex_m::Peripherals::take().unwrap();

    // Take Peripherals from Chip
    let p = mkl25z4::Peripherals::take().unwrap();

    //let clocks = clocks::init();
    //watchdog::disable(&mut p.SIM); // TODO: Registereinstellug... -> bricht sonst ab.

    hprintln!("peripherals taken").unwrap();

    p.SIM.scgc5.write(|w| {
        w.porta()
            .variant(mkl25z4::sim::scgc5::PORTA_A::_1)
            .portb()
            .variant(mkl25z4::sim::scgc5::PORTB_A::_1)
            .portd()
            .variant(mkl25z4::sim::scgc5::PORTD_A::_1)
    });

    let portb = p.PORTB;
    let porta = p.PORTA;
    let portd = p.PORTD;
    let fgpioa = p.FGPIOA;
    let fgpiob = p.FGPIOB;
    let fgpiod = p.FGPIOD;
    let pcra13 = &porta.pcr[13];
    let pcrb18 = &portb.pcr[18];
    let pcrb19 = &portb.pcr[19];
    let pcrd01 = &portd.pcr[01];
    hprintln!("ports specified").unwrap();

    //pcrb18.modify(|_,w| w.mux().variant(mkl25z4::portb::pcr::MUX_A::_001) );

    //hprintln!("Jumped over?").unwrap();

    // gpioa.dir.read().pin0().variant() == gpio::dir::PIN0_A::Input { .. }

    //pcra13.modify(|_, w| w.mux().bits(0b001));

    //porta.pcr[13].write(|w| w.mux().variant(mkl25z4::porta::pcr::MUX_A::_001)); // configure A13 as GPIO w/ MUX_A variant _001
    if pcra13.read().mux().variant() != mkl25z4::porta::pcr::MUX_A::_001 {
        hprintln!("Start setting mux on PRC A 13").unwrap();
        pcra13.write(|w| {
            w.mux()
                .variant(mkl25z4::porta::pcr::MUX_A::_001)
                .irqc()
                .variant(mkl25z4::porta::pcr::IRQC_A::_1000)
        });

        let mut _reading = pcra13.read().mux().bits();
        hprintln!("PCR A13 value: {}", _reading).expect("no functiona!");
    } else {
        hprintln!("already set or jumped over").unwrap();
    }
    hprintln!("first Mux set.").unwrap();

    if pcrd01.read().mux().variant() != mkl25z4::portd::pcr::MUX_A::_001 {
        // it seems to stop here
        hprintln!("Start setting mux on PRC D 01").unwrap();
        pcrd01.write(|w| w.mux().variant(mkl25z4::portd::pcr::MUX_A::_001));
        hprintln!("Finished.").unwrap();
    } else {
        hprintln!("already set or jumped over").unwrap();
    }

    pcrb18.write(|w| w.mux().variant(mkl25z4::portb::pcr::MUX_A::_001)); // this one is using a enum :D
    pcrb19.write(|w| w.mux().variant(mkl25z4::portb::pcr::MUX_A::_001));
    hprintln!("Mux set:").unwrap();

    let mut _reading18 = pcrb18.read().mux().bits();
    hprintln!("PCR B18 value: {}", _reading18).expect("no functiona!");

    let mut _reading19 = pcrb19.read().mux().bits();
    hprintln!("PCR B19 value: {}", _reading19).expect("no functiona!");

    fgpiob.pddr.write(|w| unsafe { w.bits(0xC0000) }); // noch unsauber, sollte aber pin 18 und 19 setzen
    fgpiod.pddr.write(|w| unsafe { w.bits(0x00002) });
    hprintln!("DDR set.").unwrap();

    // Main loop.

    hprintln!("Entering main loop.").unwrap();

    loop {
        fgpiob.ptor.write(|w| unsafe { w.bits(0xC0000) });
        fgpiod.ptor.write(|w| unsafe { w.bits(0x00002) });
    }
}

use mkl25z4::interrupt;

#[interrupt]
fn PORTA() {
    hprintln!("dummy does something!").unwrap();
}
