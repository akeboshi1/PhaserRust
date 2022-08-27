use super::timer;

pub trait GameLoop {
    fn get_delay(&self)->f64;
    fn update(&self,time:f64)->f64;
}

// impl GameLoop for TestLoop{
//     fn update(&self,time:f64)->f64{
//         let now = timer::now();
//         now-time
//     }

//     fn get_delay(&self)->f64 {
//         self.delay
//     }
// }

// pub struct TestLoop{
//     delay:f64
// }
