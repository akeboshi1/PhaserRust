use gameloop::timer;

pub mod gameloop;
pub mod net;
pub fn main(){
    let connection = net::connection::connection::new();
    connection.startConnect("wss://echo.websocket.events");
}
pub fn update(val:f64){
    timer::now();
}


