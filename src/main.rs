#![no_main]
#![no_std]

use panic_halt as _;
use rtic::app;

#[app(device = stm32f2xx_hal::stm32, dispatchers = [EXTI0])]
mod app {
    use rtt_target::{rprintln, rtt_init_print};

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("init");

        print::spawn().ok();

        (Shared {}, Local {}, init::Monotonics {})
    }

    #[task]
    fn print(_: print::Context) {
        rprintln!("Hello?");
    }
}