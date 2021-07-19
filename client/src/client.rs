use std::{
    net::{IpAddr, SocketAddr},
    time::Duration,
};

use log::info;
use naia_client::{ClientConfig, ClientEvent, NaiaClient};

use naiagame_shared::{
    get_shared_config, manifest_load, shared_behavior, AuthEvent, ExampleActor, ExampleEvent,
    KeyCommand, PointActorColor,
};

const SERVER_PORT: u16 = 14191;

pub struct GameClient {
    pub client: NaiaClient<ExampleEvent, ExampleActor>,
    pawn_key: Option<u16>,
    queued_command: Option<KeyCommand>,
}

impl GameClient {
    pub fn new() -> Self {
        // Put your Server's IP Address here!, can't easily find this automatically from the browser
        let server_ip_address: IpAddr = "127.0.0.1"
            .parse()
            .expect("couldn't parse input IP address");
        let server_socket_address = SocketAddr::new(server_ip_address, SERVER_PORT);

        let mut client_config = ClientConfig::default();
        client_config.heartbeat_interval = Duration::from_secs(2);
        client_config.disconnection_timeout_duration = Duration::from_secs(5);

        let auth = ExampleEvent::AuthEvent(AuthEvent::new("charlie", "12345"));

        let client = NaiaClient::new(
            server_socket_address,
            manifest_load(),
            Some(client_config),
            get_shared_config(),
            Some(auth),
        );

        GameClient {
            client,
            pawn_key: None,
            queued_command: None,
        }
    }

    pub fn update(&mut self, w: bool, s: bool, a: bool, d: bool) {
        if let Some(command) = &mut self.queued_command {
            if w {
                command.w.set(true);
            }
            if s {
                command.s.set(true);
            }
            if a {
                command.a.set(true);
            }
            if d {
                command.d.set(true);
            }
        } else {
            self.queued_command = Some(KeyCommand::new(w, s, a, d));
        }

        // update
        while let Some(result) = self.client.receive() {
            match result {
                Ok(event) => match event {
                    ClientEvent::Connection => {
                        info!("Client connected to: {}", self.client.server_address());
                    }
                    ClientEvent::Disconnection => {
                        info!("Client disconnected from: {}", self.client.server_address());
                    }
                    ClientEvent::Tick => {
                        if let Some(pawn_key) = self.pawn_key {
                            if let Some(command) = self.queued_command.take() {
                                self.client.send_command(pawn_key, &command);
                            }
                        }
                    }
                    ClientEvent::AssignPawn(local_key) => {
                        self.pawn_key = Some(local_key);
                        info!("assign pawn");
                    }
                    ClientEvent::UnassignPawn(_) => {
                        self.pawn_key = None;
                        info!("unassign pawn");
                    }
                    ClientEvent::Command(pawn_key, command_type) => match command_type {
                        ExampleEvent::KeyCommand(key_command) => {
                            if let Some(typed_actor) = self.client.get_pawn_mut(&pawn_key) {
                                match typed_actor {
                                    ExampleActor::PointActor(actor) => {
                                        shared_behavior::process_command(&key_command, actor);
                                    }
                                }
                            }
                        }
                        _ => {}
                    },
                    _ => {}
                },
                Err(err) => {
                    info!("Client Error: {}", err);
                }
            }
        }
    }

    pub fn has_connection(&self) -> bool {
        self.client.has_connection()
    }
}
