extern crate serial;

#[cfg(windows)]
pub mod windows;

use windows::{send_reset_command, send_getpower_command, send_getrflink_command, send_setrflink_command, send_setpower_command, send_getstandardfreq_command, send_setstandardfreq_command, send_stopinv_command,  decode_protocol_data,  send_realtimeinv_command, convert_to_hex, Buf, BufFunc, send_opencw_command, send_match_epc_command, send_single_read_command, send_custom_inventory_command, send_set_session_command, send_set_select_command, send_setcustomfreq_command};

use std::{ sync::{Mutex, Arc}, collections::VecDeque, vec};
use serial::SerialPort;


pub struct Transfer{
  pub cancel_handle :Mutex<Arc<Mutex<Option<bool>>>>,
  pub uartport:Mutex<Option<String>>,
}

impl Transfer {
    pub fn new()->Self{
      Transfer { 
        cancel_handle: Mutex::new(Arc::new(Mutex::new(None))), 
        uartport: Mutex::new(None), 
      }
    }
}

const SETTINGS: serial::PortSettings = serial::PortSettings{
  baud_rate: serial::Baud115200,
  char_size: serial::Bits8,
  parity: serial::ParityNone,
  stop_bits: serial::Stop1,
  flow_control: serial::FlowNone,
};

pub trait UI: Clone + 'static{
  fn output(&self,msg:&str);
  fn enable_next_ui(&self);
  fn enable_ui(&self);
  fn get_inv_epc(&self,epc:&str);
  fn update_match_table(&self,epc:&str);
  fn repeate(&self);
}

#[derive(Clone)]
pub enum Mode {
   Auto(u8),
   Hand(u8),
}

#[derive(PartialEq)]
pub enum CompleteState {
    
    Complete(u8),
    Error(u8),
    Running(u8),

}


pub async fn uartstate_moniter<T:UI ,Q:SerialPort>(
  port:String,
  ui:&T,
  openport:&mut Q
){

  let log = "连接串口,串口号: ".to_owned() + &port;
  ui.output(&log);


  tokio::task::yield_now().await;

  let config =  openport.configure(&SETTINGS);

  if config.is_ok(){
    ui.output("连接成功");

    let state = send_reset_command(openport);
    if state == true{
      ui.output("发送复位命令成功");

      // let buf = get_uart_data(openport);
      let buf = decode_protocol_data(openport);
      if buf.is_ok(){
        ui.output("复位成功");
        ui.enable_next_ui();

        // let t:String = buf
        // .unwrap()
        // .iter()
        // .map(|x| convert_to_hex(*x))
        // .collect();
        // ui.output(&t);
  
      }
      else {
            ui.output("接收读写器消息失败");
      }
    }
    else{
      ui.output("发送复位命令失败");
    }
  }
  else {
    ui.output("配置失败");
  }

}

pub async fn stop_inv_epc<T:UI>(
  ui:&T,
  sta:Arc<Mutex<Option<bool>>>
){

  ui.output("停止盘存");

  let mut handle_value = sta.lock().expect("Couldn't lock hotspot mutex");
  *handle_value = Some(true);

}


pub async fn start_inv_epc<T:UI,Q:SerialPort>(
ui:&T,
openport:&mut Q,
sta:Arc<Mutex<Option<bool>>>
)->CompleteState{
    ui.output("开始实时盘存");


  tokio::task::yield_now().await;

  let config =  openport.configure(&SETTINGS);

  if config.is_err(){
    ui.output("配置失败");
    fault_state(ui);
    return CompleteState::Error(1);
  }

  ui.output("连接成功");

  // let state = reader_base_config(ui,openport);
  // if state == CompleteState::Error(1){
  //   fault_state(ui);
  //   return CompleteState::Error(1);
  // }

  // let state = jump_freq(ui, openport, 0, 1);
  // if state == false{
  //   fault_state(ui);
  //   return CompleteState::Error(1);
  // }

  let state = send_realtimeinv_command(openport);
  ui.output("发送实时盘存命令");
  if state == false{
      ui.output("发送实时盘存命令失败");
      return CompleteState::Error(1);
  }
  let data_temp:VecDeque<u8> = VecDeque::new();
  let pack:Vec<u8> = Vec::new();
  let pack_len_temp:u8 = 0;

  let mut inv_epc = Buf{ data_que: data_temp,pack, pack_len: pack_len_temp };
  loop {
      let buf = inv_epc.decode_inv_epc_data(openport);
       if buf == false{
        
      }
      loop {
          
        let temp = inv_epc.get_single_pack(ui);
        if temp.is_ok(){
          let t:String = temp
            .unwrap()
            .iter()
            .map(|x| convert_to_hex(*x))
            .collect();
            ui.get_inv_epc(&t);
        }
        else {
            break;
        }
      }

      let mut handle_value = sta.lock().expect("Couldn't lock hotspot mutex");
      let t = *handle_value;
      if t == Some(true){
        *handle_value = Some(false);

      let state = send_reset_command(openport);
      if state == true{
        ui.output("发送复位命令成功");
      }
        // let temp = inv_epc.get_single_pack(ui);
        // if temp.is_ok(){
        // ui.output("复位成功");
        // ui.enable_next_ui();
        // }

        complete_state(ui);
        break;
      }

  }




  CompleteState::Complete(1)

}


pub async fn start_match<T:UI,Q:SerialPort>(
  epc:String,
  ui:&T,
  openport:&mut Q,
  sta:Arc<Mutex<Option<bool>>>
)->CompleteState{

  ui.output("开始匹配端口");

  ui.output(&epc);

  let epc_op = hex_to_bytes(&epc);
  if epc_op == None{
    ui.output("匹配的EPC错误");
    fault_state(ui);
    return CompleteState::Error(1);
  }
  let epc_u8_orange = epc_op.unwrap();
  // let epc_list = epc_u8_orange.clone();

  tokio::task::yield_now().await;

  let config =  openport.configure(&SETTINGS);

  if config.is_err(){
    ui.output("配置失败");
    fault_state(ui);
    return CompleteState::Error(1);
  }

  ui.output("连接成功");


  // let state = jump_freq(ui, openport, 910000, 0);
  // if state == false{
  //   fault_state(ui);
  //   return CompleteState::Error(1);
  // }


  let state = send_set_select_command(openport,epc_u8_orange);
  ui.output("设置select");
  if state == false{
    ui.output("设置select失败");
    fault_state(ui);
    return  CompleteState::Error(1);
  }

  let buf = decode_protocol_data(openport);
  if buf.is_err(){
    ui.output("设置select失败");
    fault_state(ui);
    return CompleteState::Error(1);
  }


  
  let round:u8 = 0x2;
  let mode:u8 = 0x0A;
  let len:u8 = 1;
  let state = send_custom_inventory_command(openport, round, mode, len);
  ui.output("发送匹配端口命令");
  if state == false{
    ui.output("发送端口匹配失败");
    fault_state(ui);
    return  CompleteState::Error(1);
  }
  
  let data_temp:VecDeque<u8> = VecDeque::new();
  let pack:Vec<u8>= Vec::new();
  let pack_len_temp:u8 = 0;

  let mut inv_epc = Buf{ data_que: data_temp, pack :pack, pack_len: pack_len_temp };

  loop {
      
      let buf = inv_epc.decode_inv_epc_data(openport);
       if buf == false{
        // break;
      }
      loop {
          
        let temp = inv_epc.get_single_pack(ui);
        if temp.is_ok(){
          let mut value = temp.unwrap();
          if value.len() > 1{
            let len = value[0];
            let value1 = value.pop();
            let value2 = value.pop();
            if value1 == Some(0x00) && value2 == Some(0x00){
              let mut epc:Vec<u8> = Vec::new();
              for i in 1..len{
                epc.push(value[i as usize]);
              }
                let t:String = epc
                  .iter()
                  .map(|x| convert_to_hex(*x))
                  .collect();
                  ui.update_match_table(&t);

                  // let state = send_stopinv_command(openport);
                  // ui.output("发送停止盘存命令");
                  // if state == false{
                  //     ui.output("发送停止盘存命令失败");
                  // }
                  // ui.repeate();
                  // complete_state(ui);               
                  // return CompleteState::Complete(1);
            }
          }
          else {
              if value[0] == 0x12{
                  complete_state(ui);               
                  return CompleteState::Complete(1);
              }
          }

          
          // let t:String = temp
          //   .unwrap()
          //   .iter()
          //   .map(|x| convert_to_hex(*x))
          //   .collect();
          //   ui.output(&t);
        }
        else {
            break;
        }

      }
      let mut handle_value = sta.lock().expect("Couldn't lock hotspot mutex");
      let t = *handle_value;
      if t == Some(true){
        *handle_value = Some(false);

        let state = send_stopinv_command(openport);
        ui.output("发送停止盘存命令");
        if state == false{
            ui.output("发送停止盘存命令失败");
        }
        complete_state(ui);
        break;
      }

  }
  complete_state(ui);
  CompleteState::Complete(1)
}


fn hex_to_bytes(s:&str)->Option<Vec<u8>>{

  if s.len() % 2 == 0{

    (0..s.len())
          .step_by(2)
          .map(|i| s.get(i..i+2)
                          .and_then(|sub| u8::from_str_radix(sub, 16).ok()))
          .collect()
          
  }
  else {
      None
  }
}




fn reader_base_config<T:UI,Q:SerialPort>(ui:&T,openport:&mut Q)->CompleteState {


  let state = send_getrflink_command(openport);
  ui.output("获取当前射频链路");

  if state == false{
    ui.output("发送获取射频链路命令失败");
    // fault_state(ui);
    return CompleteState::Error(1);
  }

  let buf = decode_protocol_data(openport);

  if buf.is_err(){
    ui.output("获取射频链路失败");
    // fault_state(ui);
    return CompleteState::Error(1);
  }
  // let t:String = buf
  // .unwrap()
  // .iter()
  // .map(|x| convert_to_hex(*x))
  // .collect();
  // ui.output(&t);
  let rflink = buf.unwrap();
  if rflink[0] != 0xD1{
    let state = send_setrflink_command(openport, 0xD1);
    ui.output("设置射频链路");

    if state == false{
      ui.output("发送设置射频链路命令失败");
      // fault_state(ui);
      return CompleteState::Error(1);
    }

    let buf = decode_protocol_data(openport);
    if buf.is_err(){
      ui.output("设置射频链路失败");
      // fault_state(ui);
      return CompleteState::Error(1);
    }
  }

  let state = send_getpower_command(openport);
  ui.output("获取当前功率");
  if state == false{
    ui.output("发送获取功率命令失败");
    // fault_state(ui);
    return CompleteState::Error(1);
  }

  let buf = decode_protocol_data(openport);
  if buf.is_err(){
    ui.output("获取当前功率失败");
    // fault_state(ui);
    return CompleteState::Error(1);
  }
  // let t:String = buf
  // .unwrap()
  // .iter()
  // .map(|x| convert_to_hex(*x))
  // .collect();
  // ui.output(&t);
 
  let power = buf.unwrap();
  if power[0] != 20{
    let state = send_setpower_command(openport, 18);
    ui.output("设置当前功率");
    if state == false{
      ui.output("发送设置功率命令失败");
      // fault_state(ui);
      return CompleteState::Error(1);
    }

    let buf = decode_protocol_data(openport);
    if buf.is_err(){
      ui.output("设置当前功率失败");
      // fault_state(ui);
      return CompleteState::Error(1);
    }
  }

  let state = send_opencw_command(openport);
  ui.output("打开CW波");
  if state == false{
    ui.output("打开CW波失败");
    // fault_state(ui);
    return  CompleteState::Error(1);
  }

  let buf = decode_protocol_data(openport);
  if buf.is_err(){
    ui.output("打开CW波失败");
    // fault_state(ui);
    return CompleteState::Error(1);
  }

  let state = send_set_session_command(openport);
  ui.output("设置session");
  if state == false{
    ui.output("设置session失败");
    // fault_state(ui);
    return  CompleteState::Error(1);
  }

  let buf = decode_protocol_data(openport);
  if buf.is_err(){
    ui.output("设置session失败");
    // fault_state(ui);
    return CompleteState::Error(1);
  }

  // let state = send_set_select_command(openport);
  // ui.output("设置select");
  // if state == false{
  //   ui.output("设置select失败");
  //   // fault_state(ui);
  //   return  CompleteState::Error(1);
  // }

  // let buf = decode_protocol_data(openport);
  // if buf.is_err(){
  //   ui.output("设置select失败");
  //   // fault_state(ui);
  //   return CompleteState::Error(1);
  // }

  

  CompleteState::Complete(1) 
}


fn jump_freq<T:UI,Q:SerialPort>(ui:&T,openport:&mut Q,freq:u32,mode:u8)->bool{

  if mode == 0x00{
    let state = send_setcustomfreq_command(openport, freq); 
    ui.output("设置固定频点");
    if state == false{
      ui.output("发送设置当前频率命令失败");
      // fault_state(ui);
      return false;
    }
  }
  else {
      
    let state = send_setstandardfreq_command(openport, 0x01, 0x07, 0x3B);
    ui.output("设置标准频率");
    if state ==false{

      ui.output("发送设置当前频率命令失败");
      // fault_state(ui);
      return false;
    }

  }

    let buf = decode_protocol_data(openport);
    if buf.is_err(){
      ui.output("设置当前频率失败");
      // fault_state(ui);
      return false;
    }

    return true


  
}

fn fault_state<T:UI>(ui:&T){
  // std::thread::sleep(std::time::Duration::from_secs(1));
  ui.enable_ui();
}

fn complete_state<T:UI>(ui:&T){
  // std::thread::sleep(std::time::Duration::from_secs(1));
  ui.enable_ui();
}

// fn auto_fault_state<T:UI>(ui:&T){
//   ui.output("fault");
//   std::thread::sleep(std::time::Duration::from_secs(1));
//   ui.enable_ui();
// }

// fn auto_complete_state<T:UI>(ui:&T){
//   ui.output("success");
//   std::thread::sleep(std::time::Duration::from_secs(1));
//   ui.enable_ui();
// }

