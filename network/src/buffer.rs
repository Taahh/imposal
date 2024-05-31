pub struct Buffer {
    pub data: Vec<u8>,
    index_stack: Vec<usize>,
    index: usize
}

impl Buffer {
    pub fn from(data: Vec<u8>) -> Buffer {
        return Buffer {
            data,
            index_stack: vec![],
            index: 0,
        }
    }

    pub fn read_byte(mut self) -> u8 {
        let val = self.data[self.index];
        self.index += 1;
        return val;
    }

    pub fn read_short(mut self) -> i16 {
        let data = self.data[self.index..self.index+2].try_into().unwrap();
        let val = i16::from_be_bytes(data);
        self.index += 2;
        return val;
    }

    pub fn read_int(mut self) -> i32 {
        let data: [u8; 4] = self.data[self.index..self.index+4].try_into().unwrap();
        let val = i32::from_be_bytes(data);
        self.index += 4;
        return val;
    }

    pub fn read_int_le(mut self) -> i32 {
        let data: [u8; 4] = self.data[self.index..self.index+4].try_into().unwrap();
        let val = i32::from_le_bytes(data);
        self.index += 4;
        return val;
    }

    pub fn read_long(mut self) -> i64 {
        let data: [u8; 8] = self.data[self.index..self.index+8].try_into().unwrap();
        let val = i64::from_be_bytes(data);
        self.index += 8;
        return val;
    }
}