use std::convert::TryFrom;
use std::convert::TryInto;

pub mod client_to_server;
pub mod server_to_client;

use crate::json_rpc::Message;

pub enum Method {
    Client2Server(Client2Server),
    Server2Client(Server2Client),
    Server2ClientResponse(Server2ClientResponse),
}

pub enum Client2Server {
    Subscribe(client_to_server::Subscribe),
    Authorize(client_to_server::Authorize),
    ExtranonceSubscribe(client_to_server::ExtranonceSubscribe),
    Submit(client_to_server::Submit),
    Configure(client_to_server::Configure),
}

impl TryFrom<Message> for Client2Server {
    type Error = ();

    fn try_from(msg: Message) -> Result<Self, ()> {
        let method: Method = msg.try_into()?;
        match method {
            Method::Client2Server(client_to_server) => Ok(client_to_server),
            Method::Server2Client(_) => Err(()),
            Method::Server2ClientResponse(_) => Err(()),
        }
    }
}

pub enum Server2Client {
    Notify(server_to_client::Notify),
    SetDifficulty(server_to_client::SetDifficulty),
    SetExtranonce(server_to_client::SetExtranonce),
    SetVersionMask(server_to_client::SetVersionMask),
}

impl TryFrom<Message> for Server2Client {
    type Error = ();

    fn try_from(msg: Message) -> Result<Self, ()> {
        let method: Method = msg.try_into()?;
        match method {
            Method::Server2Client(client_to_server) => Ok(client_to_server),
            Method::Client2Server(_) => Err(()),
            Method::Server2ClientResponse(_) => Err(()),
        }
    }
}

pub enum Server2ClientResponse {
    Configure(server_to_client::Configure),
    Subscribe(server_to_client::Subscribe),
    Authorize(server_to_client::Authorize),
    Submit(server_to_client::Submit),
}

impl TryFrom<Message> for Server2ClientResponse {
    type Error = ();

    fn try_from(msg: Message) -> Result<Self, ()> {
        let method: Method = msg.try_into()?;
        match method {
            Method::Server2ClientResponse(server_to_client) => Ok(server_to_client),
            Method::Client2Server(_) => Err(()),
            Method::Server2Client(_) => Err(()),
        }
    }
}

impl TryFrom<Message> for Method {
    type Error = ();

    fn try_from(msg: Message) -> Result<Self, ()> {
        match msg {
            Message::StandardRequest(msg) => match &msg.method[..] {
                "mining.subscribe" => {
                    let method = msg.try_into()?;
                    Ok(Method::Client2Server(Client2Server::Subscribe(method)))
                }
                "mining.authorize" => {
                    let method = msg.try_into()?;
                    Ok(Method::Client2Server(Client2Server::Authorize(method)))
                }
                "mining.extranonce.subscribe" => Ok(Method::Client2Server(
                    Client2Server::ExtranonceSubscribe(client_to_server::ExtranonceSubscribe()),
                )),
                "mining.submit" => {
                    let method = msg.try_into()?;
                    Ok(Method::Client2Server(Client2Server::Submit(method)))
                }
                "mining.configure" => {
                    let method = msg.try_into()?;
                    Ok(Method::Client2Server(Client2Server::Configure(method)))
                }
                _ => Err(()),
            },
            Message::Notification(msg) => match &msg.method[..] {
                "mining.notify" => {
                    let method = msg.try_into()?;
                    Ok(Method::Server2Client(Server2Client::Notify(method)))
                }
                "mining.set_version_mask" => {
                    let method = msg.try_into()?;
                    Ok(Method::Server2Client(Server2Client::SetVersionMask(method)))
                }
                "mining.set_difficulty" => {
                    let method = msg.try_into()?;
                    Ok(Method::Server2Client(Server2Client::SetDifficulty(method)))
                }
                "mining.set_extranonce" => {
                    let method = msg.try_into()?;
                    Ok(Method::Server2Client(Server2Client::SetExtranonce(method)))
                }
                _ => Err(()),
            },
            Message::Response(msg) => {
                if msg.error.is_some() {
                    todo!()
                } else {
                    let subscribe: Result<server_to_client::Subscribe, ()> = (&msg).try_into();
                    let configure: Result<server_to_client::Configure, ()> = (&msg).try_into();
                    match (subscribe, configure) {
                        (Ok(a), Err(_)) => Ok(Method::Server2ClientResponse(
                            Server2ClientResponse::Subscribe(a),
                        )),
                        (Err(_), Ok(a)) => Ok(Method::Server2ClientResponse(
                            Server2ClientResponse::Configure(a),
                        )),
                        (Ok(_), Ok(_)) => Err(()),
                        (Err(_), Err(_)) => Err(()),
                    }
                }
            }
        }
        //res
    }
}
