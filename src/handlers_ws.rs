use actix::{Actor, StreamHandler, AsyncContext, ActorContext};
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;

// use actix::ActorContext;


pub struct WsSession {
    pub workspace: String,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connected to workspace [{}]", self.workspace);
        ctx.text(format!("Connected to workspace: {}", self.workspace));
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("Message from [{}]: {}", self.workspace, text);
                ctx.text(format!("Echo: {}", text));
            }
            Ok(ws::Message::Close(reason)) => {
                println!("Closing WS for workspace [{}]: {:?}", self.workspace, reason);
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}

pub async fn handler(req: HttpRequest, stream: web::Payload, path: web::Path<String>) -> Result<HttpResponse, Error> {
    let workspace = path.into_inner();
    let session = WsSession { workspace };
    ws::start(session, &req, stream)
}
