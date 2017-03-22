extern crate libc;
use self::libc::{c_int, c_uint};

// from pcm_format in tinyalsa
#[repr(C)]
#[derive(Copy, Clone)]
pub enum PcmFormat {
    PcmFormatS16BitLe = 0,
    PcmFormatS8Bit,
    PcmFormatS16BitBe,
    PcmFormatS24BitLe,
    PcmFormatS24BitBe,
    PcmFormatS24Bit3Le,
    PcmFormatS24Bit3Be,
    PcmFormatS32BitLe,
    PcmFormatS32BitBe,
    PcmFormatMax,
}

// From pcm_config in tinyalsa
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PcmConfig {
    channels: c_uint,
    rate: c_uint,
    period_size: c_uint,
    period_count: c_uint,
    format: PcmFormat,
    start_threshold: c_uint,
    stop_threshold: c_uint,
    silence_threshold: c_uint,
}

pub enum Pcm {} // struct pcm in tinyalsa

#[link(name = "tinyalsa")]
extern {
    pub fn pcm_open(card: c_uint, device: c_uint, flags: c_uint, config: *const PcmConfig) -> Option<&mut Pcm>;
    pub fn pcm_close(pcm: *mut Pcm) -> c_int;
}

#[cfg(test)]
mod tests {
    use super::{pcm_close, pcm_open, PcmConfig, PcmFormat};

    #[test]
    fn basic_open() {
        let config = PcmConfig {
            channels: 2,
            rate: 48000,
            period_size: 512,
            period_count: 2,
            format: PcmFormat::PcmFormatS16BitLe,
            start_threshold: 0,
            stop_threshold: 0,
            silence_threshold: 0,
        };

        unsafe {
            let pcm = pcm_open(1, 0, 0, &config);
            assert_eq!(0, pcm_close(pcm.unwrap()));
        }
    }
}
