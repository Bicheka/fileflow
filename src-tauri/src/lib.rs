use file_transfer_system::client::Client;
use file_transfer_system::network::Request;
use file_transfer_system::p2p::upnp::upnp;
use file_transfer_system::server::Server;
use file_transfer_system::{network, server};
use tauri::State;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::{net::IpAddr, str::FromStr};
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};

pub struct GlobalState {
    server: Arc<Mutex<Option<server::Server>>>,
    stop_signal: Arc<Notify>,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            server: Arc::new(Mutex::new(None)),
            stop_signal: Arc::new(Notify::new()),
        }
    }

    pub async fn get_server(&self) -> Arc<Mutex<Option<server::Server>>> {
        self.server.clone()
    }

    pub fn get_stop_signal(&self) -> Arc<Notify> {
        Arc::clone(&self.stop_signal)
    }
}

pub struct ClientState {
    client: Arc<Mutex<Option<Client>>>
}

impl ClientState {
    pub fn new() -> Self {
        Self {
            client: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn get_client(&self) -> Arc<Mutex<Option<Client>>> {
        self.client.clone()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let global_state = GlobalState::new();
    let client_state = ClientState::new();
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .manage(global_state)
        .manage(client_state)
        .invoke_handler(tauri::generate_handler![
            start_server, 
            stop_server, start_client, 
            connect,
            send,
            download
        ])
        .setup(|_app| {
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .run(tauri::generate_context!())
        .expect("error while running the application");
}

#[tauri::command]
async fn start_server(global_state: State<'_, GlobalState>, path: &str) -> Result<String, String> {
    create_server(&global_state, path).await.unwrap();

    let state_arc = global_state.get_server().await;
    let arc_clone = Arc::clone(&state_arc);

    tokio::spawn(async move {
        let mut lock = arc_clone.lock().await;
        if let Some(server) = lock.as_mut() {
            server.start_server().await.unwrap()
        }
    });

    Ok("Server Running".to_string())
}

async fn create_server(global_state: &State<'_, GlobalState>, path: &str) -> Result<(), String> {
    let stop_signal = global_state.get_stop_signal();
    let stop_signal_clone = Arc::clone(&stop_signal);

    let port: u16 = 8080;
    let ip = if let Ok(ip) = upnp(port).await {
        println!("Public IP: {}", ip);
        network::get_local_ip().expect("failed to get local IP address")
    } else {
        println!("Continuing using IPv6");
        network::get_public_ip(network::IpType::IPv6)
            .await
            .expect("failed to start server with IPv6")
    };
    let state_arc = global_state.get_server().await;
    let mut arc_clone = state_arc.lock().await;
    if arc_clone.is_none() {
        let server = Server::new(ip, port, path, 4096, stop_signal_clone);
        *arc_clone = Some(server);
    } else {
        println!("server already exists")
    }
    Ok(())
}

#[tauri::command]
async fn stop_server(global_state: State<'_, GlobalState>) -> Result<(), String> {
    println!("Trying to stop server");
    // Notify the stop signal
    global_state.get_stop_signal().notify_waiters(); // Notify the server to stop
    Ok(())
}
#[tauri::command]
async fn start_client(client_state: State<'_, ClientState>, server_address: &str, local_path: &str) -> Result<(), String> {
    let client = client_state.get_client().await;
    let mut client = client.lock().await;
    let server_address = IpAddr::from_str(server_address).unwrap();
    let c = Client::new(local_path, server_address);
    match *client {
        Some(_) => Err("Server already exists".to_owned()),
        None => {
            *client = Some(c);
            println!("client started with address: {} and local path: {}", server_address, local_path);
            Ok(())
        },
    }
}

#[tauri::command]
async fn connect(client_state: State<'_, ClientState>) -> Result<(), String> {
    let mut client = client_state.client.lock().await;
    if let Some(c) = client.as_mut() {
        c.connect().await.map_err(|e| e.to_string())?;
        println!("Connected to server");
        Ok(())
    } else {
        Err("Server is none".to_owned())
    }
}

/// checks if Client exists then sends a request to the server if it is ok proceed to send
#[tauri::command]
async fn send(client_state: State<'_, ClientState>, path_to_send: &str) -> Result<(), String> {
    let request = Request::Upload;
    let client = client_state.client.lock().await;
    if let Some(c) = client.as_ref() {
        c.send_request(request).await.map_err(|e| e.to_string())?;
        match c.send(path_to_send).await {
            Ok(_) => Ok(()),
            Err(_) => Err("could not send file/s".to_owned()),
        }
    } else {
        Err("Server is none".to_owned())
    }
}
/// checks if Client exists then sends a request to the server if it is ok proceed to download
#[tauri::command]
async fn download(client_state: State<'_, ClientState>, path_to_download: &str) -> Result<(), String> {
    let request = Request::Get(path_to_download.to_owned());
    let client = client_state.client.lock().await;
    if let Some(c) = client.as_ref() {
        c.send_request(request).await.map_err(|e| e.to_string())?;
        println!("Request accepted");
        match c.download().await {
            Ok(_) => Ok(()),
            Err(_) => Err("could not download file/s".to_owned()),
        }
    } else {
        Err("Server is none".to_owned())
    }
}
