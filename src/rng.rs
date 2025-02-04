use embedded_hal::blocking::rng::Read;
use esp8266::RNG;
use void::Void;

pub struct Rng {
    rng: RNG,
}

impl Read for Rng {
    type Error = Void;

    fn read(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        for byte in buffer.iter_mut() {
            *byte = self.rng.rng().read().bits() as u8;
        }

        Ok(())
    }
}
