
pub struct RdbCount {
    pub modify_statistics: u64,
}

impl RdbCount {

    pub fn new() -> RdbCount{

        RdbCount {
            modify_statistics: 0
        }
    }

    pub fn calc(&mut self) {
        self.modify_statistics += 1;
    }  

    pub fn init(&mut self) {
        self.modify_statistics = 0;
    }
}