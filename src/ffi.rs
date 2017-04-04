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
    channels: ::libc::c_uint,
    rate: ::libc::c_uint,
    period_size: ::libc::c_uint,
    period_count: ::libc::c_uint,
    format: PcmFormat,
    start_threshold: ::libc::c_uint,
    stop_threshold: ::libc::c_uint,
    silence_threshold: ::libc::c_uint,
}

// All flags from pcm.h in tinyalsa.
const PCM_OUT: ::libc::c_uint = 0x00000000;
const PCM_IN: ::libc::c_uint = 0x10000000;
const PCM_MMAP: ::libc::c_uint = 0x00000001;
const PCM_NOIRQ: ::libc::c_uint = 0x00000002;
const PCM_NORESTART: ::libc::c_uint = 0x00000004;
const PCM_MONOTONIC: ::libc::c_uint = 0x00000008;

pub enum Pcm {} // struct pcm in tinyalsa

#[link(name = "tinyalsa")]
extern {
    pub fn pcm_open(card: ::libc::c_uint, device: ::libc::c_uint, flags: ::libc::c_uint,
                    config: *const PcmConfig) -> Option<&mut Pcm>;
    pub fn pcm_close(pcm: *mut Pcm) -> ::libc::c_int;
    pub fn pcm_is_ready(pcm: *const Pcm) -> ::libc::c_int;

    pub fn pcm_writei(pcm: *mut Pcm, data: *const ::libc::c_void,
                      frames: ::libc::c_uint) -> ::libc::c_int;
    pub fn pcm_readi(pcm: *mut Pcm, data: *mut ::libc::c_void,
                     frames: ::libc::c_uint) -> ::libc::c_int;
}

#[cfg(test)]
mod tests {
    use super::*;

    const PCM_CARD: u32 = 1;
    const PCM_DEV: u32 = 0;

    const PCM_DEV_CONFIG: PcmConfig = PcmConfig {
        channels: 2,
        rate: 48000,
        period_size: 512,
        period_count: 2,
        format: PcmFormat::PcmFormatS16BitLe,
        start_threshold: 0,
        stop_threshold: 0,
        silence_threshold: 0,
    };

    #[test]
    fn basic_open() {
        unsafe {
            let pcm = pcm_open(PCM_CARD, PCM_DEV, 0, &PCM_DEV_CONFIG).unwrap();
            assert_eq!(1, pcm_is_ready(pcm));
            assert_eq!(0, pcm_close(pcm));
        }
    }

    #[test]
    fn basic_writei() {
        unsafe {
            let zero_buf: [i16; 1024] = [0; 1024];
            let pcm = pcm_open(PCM_CARD, PCM_DEV, 0, &PCM_DEV_CONFIG).unwrap();
            assert_eq!(1, pcm_is_ready(pcm));
	    assert_eq!(512, pcm_writei(pcm,
                                       &zero_buf[0] as *const _ as *const ::libc::c_void,
                                       512));
            assert_eq!(0, pcm_close(pcm));
        }
    }

    #[test]
    fn basic_readi() {
        unsafe {
            let mut zero_buf: [i16; 1024] = [0; 1024];
            let pcm = pcm_open(PCM_CARD, PCM_DEV, PCM_IN, &PCM_DEV_CONFIG).unwrap();
            assert_eq!(1, pcm_is_ready(pcm));
	    assert_eq!(512, pcm_readi(pcm,
                                      &mut zero_buf[0] as *mut _ as *mut ::libc::c_void,
                                      512));
            assert_eq!(0, pcm_close(pcm));
        }
    }
}
