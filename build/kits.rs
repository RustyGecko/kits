extern crate gcc;
extern crate submodules;

use gcc::Config;

use std::env;

fn main() {
    submodules::update()
        .init()
        .recursive()
        .run();

    compile_library();
}

fn compile_library() {
    println!("The ARM embedded toolchain must be available in the PATH");
    env::set_var("CC", "arm-none-eabi-gcc");
    env::set_var("AR", "arm-none-eabi-ar");

    let mut config = Config::new();

    kit_config(&mut config)
        .define("EFM32GG990F1024", None)

        .flag("-mcpu=cortex-m3")
        .flag("-mthumb")

        .include("efm32-common/CMSIS/Include")
        .include("efm32-common/Device/EFM32GG/Include")
        .include("efm32-common/emlib/inc")
        .include("efm32-common/kits/common/bsp/")
        .file("efm32-common/kits/common/bsp/bsp_trace.c")

        .compile("libkits.a");
}

#[cfg(feature = "dk3750")]
pub fn kit_config(config: &mut Config) -> &mut Config {
    println!("Configuring DK3750");
    config
        .include("efm32-common/kits/EFM32GG_DK3750/config")

        .file("efm32-common/kits/common/bsp/bsp_dk_3201.c")
        .file("efm32-common/kits/common/bsp/bsp_dk_leds.c")

        .file("src/dk/bc/get_bc_register.c")
}

#[cfg(feature = "stk3700")]
pub fn kit_config(config: &mut Config) -> &mut Config {
    println!("Configuring STK3700");
    config
        .include("efm32-common/kits/EFM32GG_STK3700/config")

        .file("efm32-common/kits/common/bsp/bsp_stk.c")
        .file("efm32-common/kits/common/bsp/bsp_bcc.c")

}
