use actix::{Actor, StreamHandler};
use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Error};
use actix_web_actors::ws;


struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("{:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) if text == "ping" => ctx.text(String::from("pong")),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}\n", resp);
    resp
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");
    let res = HttpServer::new(|| App::new().route("/ws/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await;
    println!("Bye!");
    res
}