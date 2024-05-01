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
