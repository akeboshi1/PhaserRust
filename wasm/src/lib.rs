use gameloop::timer;

pub mod gameloop;
pub mod net;
pub fn main(){
    let connection = net::connection::Connection::new();
    connection.start_connect("wss://echo.websocket.events");
}
pub fn update(val:f64){
    timer::now();
}


