// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


use std::sync::{Arc, Mutex};

// use serial::windows::COMPort;
use tauri::{Window, State};

use project_core::{
   UI, 
    uartstate_moniter, Transfer, start_inv_epc, stop_inv_epc, start_match,
};



#[derive(Clone,serde::Serialize)]
struct Payload{
  message : String,
}

#[derive(Clone,serde::Serialize)]
struct Progress{
  value : u8,
}
#[derive(Clone)]
struct GUI{
  window: Arc<Mutex<Window>>,
}

impl UI for GUI{
    fn output(&self,msg:&str) {
      self.window
          .lock()
          .expect("Couldn't lock GUI mutex")
          .emit(
            "outputMsg",
            Payload{
              message:msg.to_string(),
            },
          )
          .expect("could not emit event");
    }
    fn enable_next_ui(&self){
        self.window
            .lock()
            .expect("coundn't lock GUI mutex")
            .emit("enableNextUi", Progress{value:0})
            .expect("coundn't not emit event");
    }
    fn enable_ui(&self){
        self.window
            .lock()
            .expect("coundn't lock GUI mutex")
            .emit("enableUi", Progress{value:0})
            .expect("coundn't not emit event");
    }
    fn get_inv_epc(&self,epc:&str){
      self.window
          .lock()
          .expect("Couldn't lock GUI mutex")
          .emit(
            "getinvEPC",
            Payload{
              message:epc.to_string(),
            },
          )
          .expect("could not emit event");
    }
    fn update_match_table(&self,epc:&str){
      self.window
          .lock()
          .expect("Couldn't lock GUI mutex")
          .emit(
            "setMatchEPC",
            Payload{
              message:epc.to_string(),
            },
          )
          .expect("could not emit event");
    }
}


#[tauri::command]
fn connect_uart(
  sta:State<Transfer>,
  comport:String,
  window:Window
){
  let thread_window = window.clone();
  let gui = GUI{
    window: Arc::new(Mutex::new(thread_window)),
  };

  let  openport = serial::open(&comport);

  let op_port = Some(comport.clone());

  if openport.is_ok(){
    let _cancel_handle = tokio::spawn(async move{
        uartstate_moniter(comport, &gui,&mut openport.unwrap()) .await;
    });
    let port = &mut sta.uartport.lock().unwrap();
    **port = op_port;
  }
  else{
        window
        .emit("outputMsg", Payload { message: "连接失败".to_string() })
        .expect("Couldn't emit to window");
  }
}

#[tauri::command]
fn inventory_epc(
  sta:State<Transfer>,
  window:Window
){
  let thread_window = window.clone();
  let gui = GUI{
    window: Arc::new(Mutex::new(thread_window)),
  };
  let muport = sta.uartport.lock().unwrap();

  if let Some(comport) = muport.as_ref(){
    let openport = serial::open(&comport);
    if openport.is_ok(){
      let cancel_handle = sta.cancel_handle.lock().expect("Couldn't lock hotspot mutex.");
      let transfer_handle = cancel_handle.clone();
      let _cancel_handle = tokio::spawn(async move{
          let _result = start_inv_epc(
                                                    &gui,
                                                    &mut openport.unwrap(),
                                                    transfer_handle
                                                    )
                                                    .await;
      });     
    }
  }
}

#[tauri::command]
fn stop_inventory(
  sta:State<Transfer>,
  window:Window
){
  let thread_window = window.clone();
  let gui = GUI{
    window: Arc::new(Mutex::new(thread_window)),
  };

  let cancel_handle = sta.cancel_handle.lock().expect("Couldn't lock hotspot mutex.");
  let transfer_handle = cancel_handle.clone();
  let _cancel_handle = tokio::spawn(async move{
      stop_inv_epc(
                                &gui,
                                transfer_handle
                                )
                                .await;
  });     
}

#[tauri::command]
fn start_match_epc(
  epc:String,
  sta:State<Transfer>,
  window:Window
){

  let thread_window = window.clone();
  let gui = GUI{
    window: Arc::new(Mutex::new(thread_window)),
  }; 

  let muport = sta.uartport.lock().unwrap();

  if let Some(comport) = muport.as_ref(){
    let openport = serial::open(&comport);
    let cancel_handle = sta.cancel_handle.lock().expect("Couldn't lock hotspot mutex.");
    let transfer_handle = cancel_handle.clone();
    if openport.is_ok(){
      let _cancel_handle = tokio::spawn(async move{
          let _result = start_match(
                                                    epc,
                                                    &gui,
                                                    &mut openport.unwrap(),
                                                    transfer_handle
                                                    )
                                                    .await;
      });     
    }
  }


}

#[tauri::command]
fn stop_match(
  sta:State<Transfer>,
  window:Window
){
  let thread_window = window.clone();
  let gui = GUI{
    window: Arc::new(Mutex::new(thread_window)),
  };

  let cancel_handle = sta.cancel_handle.lock().expect("Couldn't lock hotspot mutex.");
  let transfer_handle = cancel_handle.clone();
  let _cancel_handle = tokio::spawn(async move{
      stop_inv_epc(
                                &gui,
                                transfer_handle
                                )
                                .await;
  });     
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .manage(Transfer::new())
        .invoke_handler(tauri::generate_handler![
          connect_uart,
          inventory_epc,
          stop_inventory,
          start_match_epc,
          stop_match
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");


}
