pub struct ROM {
    data: Vec<u8>,
}

impl ROM {
    pub fn new(data: Vec<u8>) -> ROM {
        ROM { data }
    }

    pub fn read(&self, address: usize) -> u8 {
        self.data[address]
    }

    // debug
    pub fn get_buf(&self) -> Vec<u8> {
        self.data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rom_new() {
        let data = vec![0; 0x100];
        let rom = ROM::new(data.clone());
        assert_eq!(rom.get_buf(), data);
    }

    #[test]
    fn test_rom_read() {
        let data = vec![0; 0x100];
        let rom = ROM::new(data.clone());
        for i in 0..0x100 {
            assert_eq!(rom.read(i), data[i]);
        }
    }
}