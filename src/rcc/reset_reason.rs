//! A module that can capture the RCC registers and read the reason why the mcu
//! has reset

use core::fmt::Display;

/// Gets and clears the reason of why the mcu was reset
#[rustfmt::skip]
#[allow(dead_code)]
pub fn get_reset_reason(rcc: &mut crate::stm32::RCC) -> ResetReason {
    let reset_reason = rcc.rsr().read();

    // Clear the register
    rcc.rsr().modify(|_, w| w.rmvf().reset());

    #[cfg(feature = "rm0492")]
    // See R0492 Section 10.3.4 Reset source identification
    return match (
        reset_reason.lpwrrstf().is_reset_occurred(),
        reset_reason.wwdgrstf().is_reset_occurred(),
        reset_reason.iwdgrstf().is_reset_occurred(),
        reset_reason.sftrstf().is_reset_occurred(),
        reset_reason.borrstf().is_reset_occurred(),
        reset_reason.pinrstf().is_reset_occurred(),
    ) {
        (false, false, false, false, true, true) => {
            ResetReason::PowerOnReset
        }
        (false, false, false, false, false, true) => {
            ResetReason::PinReset
        }
        (false, false, false, true, false, true) => {
            ResetReason::SystemReset
        }
        (false, true, false, false, false, true) => {
            ResetReason::WindowWatchdogReset
        }
        (false, false, true, false, false, true) => {
            ResetReason::IndependentWatchdogReset
        }
        (true, false, false, false, false, true) => {
            ResetReason::IllegalStopEntryReset
        }
        (false, true, true, false, false, true) => {
            ResetReason::GenericWatchdogReset
        }
        _ => ResetReason::Unknown {
            rcc_rsr: reset_reason.bits(),
        },
    };

    #[cfg(feature = "rm0481")]
    return ResetReason::Unknown { rcc_rsr: reset_reason.bits() }

}

/// Gives the reason why the mcu was reset
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum ResetReason {
    /// The mcu went from not having power to having power and resetting
    PowerOnReset,
    /// The reset pin was asserted
    PinReset,
    /// The software did a soft reset through the AIRCR register of the M33 core
    SystemReset,
    /// The window watchdog triggered
    WindowWatchdogReset,
    /// The independent watchdog triggered
    IndependentWatchdogReset,
    /// Either or both of the two watchdogs triggered (but we don't know which one)
    GenericWatchdogReset,
    /// Illegal stop entry reset triggered
    IllegalStopEntryReset,
    /// The reason could not be determined
    Unknown {
        /// The raw register value
        rcc_rsr: u32,
    },
}

impl Display for ResetReason {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ResetReason::PowerOnReset => write!(f, "Power-on reset"),
            ResetReason::PinReset => write!(f, "Pin reset (NRST)"),
            ResetReason::SystemReset => {
                write!(f, "System reset generated by CPU (SYSRESETREQ)")
            }
            ResetReason::WindowWatchdogReset => write!(f, "WWDG reset"),
            ResetReason::IndependentWatchdogReset => write!(f, "IWDG reset"),
            ResetReason::GenericWatchdogReset => {
                write!(f, "IWDG or WWDG reset")
            }
            ResetReason::IllegalStopEntryReset => {
                write!(f, "Illegal stop entry reset")
            }
            ResetReason::Unknown { rcc_rsr } => write!(
                f,
                "Could not determine the cause. RCC RSR bits were 0x{:X}",
                rcc_rsr
            ),
        }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ResetReason {
    fn format(&self, f: defmt::Formatter<'_>) {
        match self {
            ResetReason::PowerOnReset => {
                defmt::intern!("Power-on reset").format(f)
            }
            ResetReason::PinReset => {
                defmt::intern!("Pin reset (NRST)").format(f)
            }
            ResetReason::SystemReset => {
                defmt::intern!("System reset generated by CPU (SYSRESETREQ)")
                    .format(f)
            }
            ResetReason::WindowWatchdogReset => {
                defmt::intern!("WWDG reset").format(f)
            }
            ResetReason::IndependentWatchdogReset => {
                defmt::intern!("IWDG reset").format(f)
            }
            ResetReason::GenericWatchdogReset => {
                defmt::intern!("IWDG or WWDG reset").format(f)
            }
            ResetReason::IllegalStopEntryReset => {
                defmt::intern!("Illegal stop entry reset").format(f)
            }
            ResetReason::Unknown { rcc_rsr } => defmt::write!(
                f,
                "Could not determine the cause. RCC RSR bits were 0x{:X}",
                rcc_rsr
            ),
        }
    }
}
