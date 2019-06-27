
#[derive(Debug,Clone)]
pub struct Position{
    pub x:u32,
    pub y:u32,
}

impl Position{
    pub fn range_to(&self, target:&Position) -> u32{
        let diff_x = if self.x > target.x {
            self.x - target.x
        }else{
            target.x - self.x
        };

        diff_x + if self.y > target.y{
            self.y - target.y
        }else{
            target.y - self.y
        }
    }
}