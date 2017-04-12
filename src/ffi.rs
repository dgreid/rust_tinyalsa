// from pcm_format in tinyalsa
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
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

// From alsalib asound.h
//  tinyalsa doesn't provide this, but it's needed to check if a format is valid
//  in params.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum AlsaFormatBitMask {
	/** Signed 8 bit */
	SND_PCM_FORMAT_S8 = 0,
	/** Unsigned 8 bit */
	SND_PCM_FORMAT_U8,
	/** Signed 16 bit Little Endian */
	SND_PCM_FORMAT_S16_LE,
	/** Signed 16 bit Big
	 * Endian */
	SND_PCM_FORMAT_S16_BE,
	/** Unsigned 16 bit Little Endian */
	SND_PCM_FORMAT_U16_LE,
	/** Unsigned 16 bit Big Endian */
	SND_PCM_FORMAT_U16_BE,
	/** Signed 24 bit Little Endian using low three bytes in 32-bit word */
	SND_PCM_FORMAT_S24_LE,
	/** Signed 24 bit Big Endian using low three bytes in 32-bit word */
	SND_PCM_FORMAT_S24_BE,
	/** Unsigned 24 bit Little Endian using low three bytes in 32-bit word */
	SND_PCM_FORMAT_U24_LE,
	/** Unsigned 24 bit Big Endian using low three bytes in 32-bit word */
	SND_PCM_FORMAT_U24_BE,
	/** Signed 32 bit Little Endian */
	SND_PCM_FORMAT_S32_LE,
	/** Signed 32 bit Big Endian */
	SND_PCM_FORMAT_S32_BE,
	/** Unsigned 32 bit Little Endian */
	SND_PCM_FORMAT_U32_LE,
	/** Unsigned 32 bit Big Endian */
	SND_PCM_FORMAT_U32_BE,
	/** Float 32 bit Little Endian, Range -1.0 to 1.0 */
	SND_PCM_FORMAT_FLOAT_LE,
	/** Float 32 bit Big Endian, Range -1.0 to 1.0 */
	SND_PCM_FORMAT_FLOAT_BE,
	/** Float 64 bit Little Endian, Range -1.0 to 1.0 */
	SND_PCM_FORMAT_FLOAT64_LE,
	/** Float 64 bit Big Endian, Range -1.0 to 1.0 */
	SND_PCM_FORMAT_FLOAT64_BE,
	/** IEC-958 Little Endian */
	SND_PCM_FORMAT_IEC958_SUBFRAME_LE,
	/** IEC-958 Big Endian */
	SND_PCM_FORMAT_IEC958_SUBFRAME_BE,
	/** Mu-Law */
	SND_PCM_FORMAT_MU_LAW,
	/** A-Law */
	SND_PCM_FORMAT_A_LAW,
	/** Ima-ADPCM */
	SND_PCM_FORMAT_IMA_ADPCM,
	/** MPEG */
	SND_PCM_FORMAT_MPEG,
	/** GSM */
	SND_PCM_FORMAT_GSM,
	/** Special */
	SND_PCM_FORMAT_SPECIAL = 31,
	/** Signed 24bit Little Endian in 3bytes format */
	SND_PCM_FORMAT_S24_3LE = 32,
	/** Signed 24bit Big Endian in 3bytes format */
	SND_PCM_FORMAT_S24_3BE,
	/** Unsigned 24bit Little Endian in 3bytes format */
	SND_PCM_FORMAT_U24_3LE,
	/** Unsigned 24bit Big Endian in 3bytes format */
	SND_PCM_FORMAT_U24_3BE,
	/** Signed 20bit Little Endian in 3bytes format */
	SND_PCM_FORMAT_S20_3LE,
	/** Signed 20bit Big Endian in 3bytes format */
	SND_PCM_FORMAT_S20_3BE,
	/** Unsigned 20bit Little Endian in 3bytes format */
	SND_PCM_FORMAT_U20_3LE,
	/** Unsigned 20bit Big Endian in 3bytes format */
	SND_PCM_FORMAT_U20_3BE,
	/** Signed 18bit Little Endian in 3bytes format */
	SND_PCM_FORMAT_S18_3LE,
	/** Signed 18bit Big Endian in 3bytes format */
	SND_PCM_FORMAT_S18_3BE,
	/** Unsigned 18bit Little Endian in 3bytes format */
	SND_PCM_FORMAT_U18_3LE,
	/** Unsigned 18bit Big Endian in 3bytes format */
	SND_PCM_FORMAT_U18_3BE,
	/* G.723 (ADPCM) 24 kbit/s, 8 samples in 3 bytes */
	SND_PCM_FORMAT_G723_24,
	/* G.723 (ADPCM) 24 kbit/s, 1 sample in 1 byte */
	SND_PCM_FORMAT_G723_24_1B,
	/* G.723 (ADPCM) 40 kbit/s, 8 samples in 3 bytes */
	SND_PCM_FORMAT_G723_40,
	/* G.723 (ADPCM) 40 kbit/s, 1 sample in 1 byte */
	SND_PCM_FORMAT_G723_40_1B,
	/* Direct Stream Digital (DSD) in 1-byte samples (x8) */
	SND_PCM_FORMAT_DSD_U8,
	/* Direct Stream Digital (DSD) in 2-byte samples (x16) */
	SND_PCM_FORMAT_DSD_U16_LE,
	/* Direct Stream Digital (DSD) in 4-byte samples (x32) */
	SND_PCM_FORMAT_DSD_U32_LE,
	/* Direct Stream Digital (DSD) in 2-byte samples (x16) */
	SND_PCM_FORMAT_DSD_U16_BE,
	/* Direct Stream Digital (DSD) in 4-byte samples (x32) */
	SND_PCM_FORMAT_DSD_U32_BE,
	SND_PCM_FORMAT_LAST = SND_PCM_FORMAT_DSD_U32_BE,

}

// From pcm_mask in tinyalsa
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PcmMask {
    bits: [u8; 32],
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SndPcmChannelArea {
    addr: *mut ::libc::c_void,
    first: ::libc::c_uint,
    step: ::libc::c_uint,
}

// All flags from pcm.h in tinyalsa.
pub const PCM_OUT: ::libc::c_uint = 0x00000000;
pub const PCM_IN: ::libc::c_uint = 0x10000000;
pub const PCM_MMAP: ::libc::c_uint = 0x00000001;
pub const PCM_NOIRQ: ::libc::c_uint = 0x00000002;
pub const PCM_NORESTART: ::libc::c_uint = 0x00000004;
pub const PCM_MONOTONIC: ::libc::c_uint = 0x00000008;

// Enumeration of a PCM's hardware parameters.
// Each of these parameters is either a mask or an interval.
// @ingroup libtinyalsa-pcm
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PcmParam {
    /** A mask that represents the type of read or write method available (e.g. interleaved, mmap). */
    Access,
    /** A mask that represents the @ref pcm_format available (e.g. @ref PCM_FORMAT_S32_LE) */
    Format,
    /** A mask that represents the subformat available */
    Subformat,
    /** An interval representing the range of sample bits available (e.g. 8 to 32) */
    SampleBits,
    /** An interval representing the range of frame bits available (e.g. 8 to 64) */
    FrameBits,
    /** An interval representing the range of channels available (e.g. 1 to 2) */
    Channels,
    /** An interval representing the range of rates available (e.g. 44100 to 192000) */
    Rate,
    PeriodTime,
    /** The number of frames in a period */
    PeriodSize,
    /** The number of bytes in a period */
    PeriodBytes,
    /** The number of periods for a PCM */
    Periods,
    BufferTime,
    BufferSize,
    BufferBytes,
    TickTime,
} /* enum pcm_param */

pub enum Pcm {} // struct pcm in tinyalsa
pub enum PcmParams {} // struct pcm_params in tinyalsa

#[link(name = "tinyalsa")]
extern "C" {
    pub fn pcm_open(card: ::libc::c_uint,
                    device: ::libc::c_uint,
                    flags: ::libc::c_uint,
                    config: *const PcmConfig)
                    -> Option<&mut Pcm>;
    pub fn pcm_close(pcm: *mut Pcm) -> ::libc::c_int;
    pub fn pcm_is_ready(pcm: *const Pcm) -> ::libc::c_int;

    pub fn pcm_get_channels(pcm: *const Pcm) -> ::libc::c_uint;
    pub fn pcm_get_rate(pcm: *const Pcm) -> ::libc::c_uint;
    pub fn pcm_get_format(pcm: *const Pcm) -> PcmFormat;
    pub fn pcm_get_error(pcm: *const Pcm) -> *const ::libc::c_char;
    pub fn pcm_get_subdevice(pcm: *const Pcm) -> ::libc::c_uint;
    pub fn pcm_format_to_bits(format: PcmFormat) -> ::libc::c_uint;
    pub fn pcm_frames_to_bytes(pcm: *const Pcm, frames: ::libc::c_uint) -> ::libc::c_uint;
    pub fn pcm_get_htimestamp(pcm: *mut Pcm,
                              avail: *mut ::libc::c_uint,
                              timespec: *mut ::libc::timespec)
                              -> ::libc::c_int;

    pub fn pcm_writei(pcm: *mut Pcm,
                      data: *const ::libc::c_void,
                      frames: ::libc::c_uint)
                      -> ::libc::c_int;
    pub fn pcm_readi(pcm: *mut Pcm,
                     data: *mut ::libc::c_void,
                     frames: ::libc::c_uint)
                     -> ::libc::c_int;

    pub fn pcm_params_get(card: ::libc::c_uint,
                          device: ::libc::c_uint,
                          flags: ::libc::c_uint)
                          -> Option<&mut PcmParams>;
    pub fn pcm_params_free(pcm_params: *mut PcmParams);
    pub fn pcm_params_get_mask(pcm_params: *const PcmParams, param: PcmParam) -> Option<&PcmMask>;
    pub fn pcm_params_get_min(pcm_params: *const PcmParams, param: PcmParam) -> ::libc::c_uint;
    pub fn pcm_params_get_max(pcm_params: *const PcmParams, param: PcmParam) -> ::libc::c_uint;

    pub fn pcm_link(pcm: *mut Pcm, pcm: *mut Pcm) -> ::libc::c_int;
    pub fn pcm_unlink(pcm: *mut Pcm) -> ::libc::c_int;
    pub fn pcm_prepare(pcm: *mut Pcm) -> ::libc::c_int;
    pub fn pcm_start(pcm: *mut Pcm) -> ::libc::c_int;
    pub fn pcm_stop(pcm: *mut Pcm) -> ::libc::c_int;
    pub fn pcm_avail_update(pcm: *mut Pcm) -> ::libc::c_int;
    pub fn pcm_state(pcm: *mut Pcm) -> ::libc::c_int;
    pub fn pcm_wait(pcm: *mut Pcm, timeout_ms: ::libc::c_int) -> ::libc::c_int;
    pub fn pcm_get_delay(pcm: *mut Pcm) -> ::libc::c_long;

    pub fn pcm_mmap_begin(pcm: *mut Pcm,
                          areas: *mut *const SndPcmChannelArea,
                          offset: *mut ::libc::c_uint,
                          frames: *mut ::libc::c_uint)
                          -> ::libc::c_int;
    pub fn pcm_mmap_commit(pcm: *mut Pcm,
                           offset: ::libc::c_uint,
                           frames: ::libc::c_uint)
                           -> ::libc::c_int;
    pub fn pcm_mmap_transfer(pcm: *mut Pcm,
                             buffer: *const ::libc::c_void,
                             bytes: ::libc::c_uint)
                             -> ::libc::c_int;
    pub fn pcm_mmap_write(pcm: *mut Pcm,
                          data: *const ::libc::c_void,
                          count: ::libc::c_uint)
                          -> ::libc::c_int;
    pub fn pcm_mmap_read(pcm: *mut Pcm,
                         data: *mut ::libc::c_void,
                         count: ::libc::c_uint)
                         -> ::libc::c_int;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

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
    fn basic_pcm_config() {
        unsafe {
            let pcm = pcm_open(PCM_CARD, PCM_DEV, 0, &PCM_DEV_CONFIG).unwrap();
            assert_eq!(1, pcm_is_ready(pcm));
            assert_eq!(2, pcm_get_channels(pcm));
            assert_eq!(48000, pcm_get_rate(pcm));
            assert_eq!(0, pcm_get_subdevice(pcm));
            assert_eq!(PcmFormat::PcmFormatS16BitLe, pcm_get_format(pcm));
            assert_eq!(0, pcm_close(pcm));
        }
    }

    #[test]
    fn params_test() {
        unsafe {
            let params = pcm_params_get(PCM_CARD, PCM_DEV, PCM_OUT);
            assert!(params.is_some());
            if let Some(params) = params {
                assert!(pcm_params_get_min(params, PcmParam::Rate) <= 48000);
                assert!(pcm_params_get_max(params, PcmParam::Rate) >= 48000);
                let format_mask = pcm_params_get_mask(params, PcmParam::Format);
                assert!(format_mask.is_some());
                assert_ne!(0, format_mask.unwrap().bits[0]);
                pcm_params_free(params);
            }
        }
    }

    #[test]
    fn config_error_string() {
        let mut this_config = PCM_DEV_CONFIG;
        this_config.rate = 55;
        unsafe {
            let pcm = pcm_open(PCM_CARD, PCM_DEV, 0, &this_config).unwrap();
            assert_eq!(0, pcm_is_ready(pcm));
            assert_eq!(CStr::from_bytes_with_nul(b"cannot set hw params: Invalid argument\0")
                           .unwrap(),
                       CStr::from_ptr(pcm_get_error(pcm)));
        }
    }

    #[test]
    fn format_to_bits() {
        unsafe {
            assert_eq!(16, pcm_format_to_bits(PcmFormat::PcmFormatS16BitLe));
            assert_eq!(8, pcm_format_to_bits(PcmFormat::PcmFormatS8Bit));
            assert_eq!(16, pcm_format_to_bits(PcmFormat::PcmFormatS16BitBe));
            assert_eq!(32, pcm_format_to_bits(PcmFormat::PcmFormatS24BitLe));
            assert_eq!(32, pcm_format_to_bits(PcmFormat::PcmFormatS24BitBe));
            assert_eq!(24, pcm_format_to_bits(PcmFormat::PcmFormatS24Bit3Le));
            assert_eq!(24, pcm_format_to_bits(PcmFormat::PcmFormatS24Bit3Be));
            assert_eq!(32, pcm_format_to_bits(PcmFormat::PcmFormatS32BitLe));
            assert_eq!(32, pcm_format_to_bits(PcmFormat::PcmFormatS32BitBe));
        }
    }

    #[test]
    fn frames_to_bytes() {
        unsafe {
            let pcm = pcm_open(PCM_CARD, PCM_DEV, 0, &PCM_DEV_CONFIG).unwrap();
            assert_eq!(1, pcm_is_ready(pcm));
            assert_eq!(256, pcm_frames_to_bytes(pcm, 64));
            assert_eq!(0, pcm_close(pcm));
        }
    }

    #[test]
    fn mmap_output() {
        unsafe {
            if let Some(pcm) = pcm_open(PCM_CARD, PCM_DEV, PCM_OUT | PCM_MMAP, &PCM_DEV_CONFIG) {
                assert_eq!(1, pcm_is_ready(pcm));
                assert_eq!(0, pcm_start(pcm));

                let mut areas: *const SndPcmChannelArea = ::std::ptr::null();
                let mut offset: ::libc::c_uint = 0;
                let mut frames: ::libc::c_uint = 512;
                assert_eq!(0,
                           pcm_mmap_begin(pcm,
                                          &mut areas as *mut *const SndPcmChannelArea,
                                          &mut offset,
                                          &mut frames));
                println!("offset {}, frames {}", offset, frames);
                assert_eq!(0, pcm_mmap_commit(pcm, offset, 0));
                assert_eq!(0, pcm_close(pcm));
            } else {
                assert!(false);
            }
        }
    }

    #[test]
    fn basic_writei() {
        unsafe {
            let zero_buf: [i16; 1024] = [0; 1024];
            let pcm = pcm_open(PCM_CARD, PCM_DEV, 0, &PCM_DEV_CONFIG).unwrap();
            let mut avail: ::libc::c_uint = 0;
            let mut ht = ::libc::timespec {
                tv_sec: 0,
                tv_nsec: 0,
            };
            assert_eq!(1, pcm_is_ready(pcm));
            assert_eq!(512,
                       pcm_writei(pcm, &zero_buf[0] as *const _ as *const ::libc::c_void, 512));
            assert_eq!(0, pcm_get_htimestamp(pcm, &mut avail, &mut ht));
            assert_eq!(512, avail);
            assert_eq!(0, pcm_close(pcm));
        }
    }

    #[test]
    fn basic_readi() {
        unsafe {
            let mut zero_buf: [i16; 1024] = [0; 1024];
            let pcm = pcm_open(PCM_CARD, PCM_DEV, PCM_IN, &PCM_DEV_CONFIG).unwrap();
            assert_eq!(1, pcm_is_ready(pcm));
            assert_eq!(512,
                       pcm_readi(pcm, &mut zero_buf[0] as *mut _ as *mut ::libc::c_void, 512));
            assert_eq!(0, pcm_close(pcm));
        }
    }
}
