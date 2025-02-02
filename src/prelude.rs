//! Prelude

pub use crate::delay::DelayExt as _stm32h5xx_hal_delay_DelayExt;
pub use crate::gpio::GpioExt as _stm32h5xx_hal_gpio_GpioExt;
pub use crate::icache::ICacheExt as _stm32h5xx_hal_delay_ICacheExt;
pub use crate::pwr::PwrExt as _stm32h5xx_hal_pwr_PwrExt;
pub use crate::rcc::RccExt as _stm32h5xx_hal_rcc_RccExt;
pub use crate::usb::UsbExt as _stm32h5xx_hal_usb_UsbExt;

pub use crate::time::U32Ext as _;
pub use fugit::{ExtU32 as _, RateExtU32 as _};
