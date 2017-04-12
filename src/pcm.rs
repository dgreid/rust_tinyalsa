use ::ffi;

pub type AlsaCardIndex = u32;
pub type AlsaDeviceIndex = u32;
pub type AudioRateHertz = u32;

pub enum Error {
    CheckingParams,
    InvalidSampleRate,
}

pub enum Direction {
    Input,
    Output,
}

pub struct PcmConfig<'a> {
    card: AlsaCardIndex,
    device: AlsaDeviceIndex,
    direction: Direction,
    min_rate: AudioRateHertz,
    max_rate: AudioRateHertz,
    sample_rate: AudioRateHertz,
    params: &'a mut ffi::PcmParams,
}

pub struct Pcm {
    card: AlsaCardIndex,
    device: AlsaDeviceIndex,
}

impl<'a> PcmConfig<'a> {
    fn new(card: AlsaCardIndex, device: AlsaDeviceIndex, direction: Direction)
                -> Result<PcmConfig<'a>, Error> {
        let flags = match direction {
            Direction::Input => ffi::PCM_IN,
            Direction::Output => ffi::PCM_OUT,
        };
        let mut params = match unsafe { ffi::pcm_params_get(card, device, flags) } {
            None => { return Err(Error::CheckingParams); },
            Some(p) => p,
        };

        let min = unsafe { ffi::pcm_params_get_min(params, ffi::PcmParam::Rate) };
        let max = unsafe { ffi::pcm_params_get_max(params, ffi::PcmParam::Rate) };

        Ok(PcmConfig { card: card, device: device, direction: direction,
                       min_rate: min, max_rate: max, sample_rate: 0, params: params })
    }

    pub fn set_rate(&'a mut self, rate: AudioRateHertz) -> Result<&'a mut PcmConfig, Error> {
        if rate < self.min_rate || rate > self.max_rate {
            Err(Error::InvalidSampleRate)
        } else {
            self.sample_rate = rate;
            Ok(self)
        }
    }

    pub fn to_pcm(&'a mut self) -> Result<Pcm, Error> {
        Ok(Pcm { card: self.card, device: self.device })
    }
}

impl<'a> Drop for PcmConfig<'a> {
    fn drop(&mut self) {
        unsafe { ffi::pcm_params_free(self.params); }
    }
}
