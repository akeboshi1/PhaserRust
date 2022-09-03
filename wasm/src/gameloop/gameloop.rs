use super::timer;

pub trait Loop<T> {
    type item;
    fn get_delay(&self)->T;
    fn update(&self,time:T)->T;
}
pub struct GameLoop<T>{
    delay:T
}

impl Loop<f64> for GameLoop<f64>{
    type item = f64;
    fn get_delay(&self)->f64 {
        self.delay
    }

    fn update(&self,time:f64)->f64{
        let now = timer::now();
        let tmpDelay = now-time;
        if(tmpDelay<self.delay){
            
        }
        tmpDelay
    }
}
