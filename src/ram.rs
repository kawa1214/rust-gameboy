use crate::mbc::KB;

pub struct Ram {
    pub work: [u8; 8 * KB],
    pub high: [u8; 8 * KB],
}

impl Ram {
    pub fn new() -> Ram {
        return Ram {
            work: [0; 8 * KB],
            high: [0; 8 * KB],
        };
    }
}
