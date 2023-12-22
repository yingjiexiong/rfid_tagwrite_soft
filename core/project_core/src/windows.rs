
// pub extern crate serial_core as core;
use calamine::{open_workbook, Error, Xlsx, Reader};


use std::{collections::VecDeque, ptr::addr_of_mut};

use serial::SerialPort;

use crate::UI;

// struct Protocol<S:SerialPort>{
//     openport:S,
//     cmd:Command,
//     data_que:VecDeque<u8>,
//     pack_len:u8
// }

pub enum Command {
  CmdReset,
  CmdSetPower,
  CmdSetFreq,
  CmdSetCw,
  Cmd6CRead,
  Cmd6CWrite,
  CmdCustomInv,
  Eos,
}


// impl<S:SerialPort> Protocol<S> {
//   pub fn new(openport:S)->Self{
//     Protocol { 
//               openport,
//               cmd: Command::Eos, 
//               data_que: VecDeque::new(),
//               pack_len:0,
//             }
//   }

//   fn checksun(&mut self,data:Vec<u8>)->u8{

//     let mut checksun:u8 = 0;
//     for d in data{
//       checksun = checksun.wrapping_add(d);
//     }
//     checksun = (!checksun ) + 1 ;

//     checksun
//   }
    
//   fn send_protocol_data(&mut self,cmd:u8,len:u8,mut data:Vec<u8>)->bool{

//     let mut buf:Vec<u8> = vec![0xA0,0x03+len,0x00];

//     buf.insert(3, cmd);

//     buf.append(&mut data);

//     let checksun = self.checksun(buf.clone());
//     buf.insert((len +4).into(), checksun);


//     let write = self.openport.write(&buf[..]);

//     if write.is_ok(){
//       return true;
//     }
//     else {
//       return false;
//     }
//   }   



// }



pub fn sendwrite_command<Q:SerialPort>(openport:&mut Q,area:u8,mut addr:Vec<u8>,len:u8,mut data:Vec<u8>,mut password:Vec<u8>)->bool{
  let mut buf:Vec<u8> = Vec::new();
  buf.append(&mut password);
  buf.push(area);
  buf.append(&mut addr);
  let mut len_vec = vec![0,len];
  buf.append(&mut len_vec);
  buf.append(&mut data);
  // let buf:Vec<u8> = vec![0x00,0x00,0x00,0x00,
  //                        0x01,
  //                        0x00,0x00,0x00,0x01,
  //                        0x00,0x07,
  //                        0x30,0x00,0xe2,0x00,0x01,0x02,0x03,0x04,0x05,0x06,0x07,0x08,0x09,0x0A];
  let state = send_protocol_data(openport, 0x82,11 + len*2, buf);
  state
}

pub fn send_stopinv_command<Q:SerialPort>(openport:&mut Q)->bool{
  let buf:Vec<u8> = Vec::new();
  let state = send_protocol_data(openport, 0x8C,0, buf);
  state
}

pub fn send_cleanepcbuff_command<Q:SerialPort>(openport:&mut Q)->bool{
  let buf:Vec<u8> = Vec::new();
  let state = send_protocol_data(openport, 0x93,0, buf);
  state
}

pub fn send_getepcbuff_command<Q:SerialPort>(openport:&mut Q)->bool{
  let buf:Vec<u8> = Vec::new();
  let state = send_protocol_data(openport, 0x90,0, buf);
  state
}

pub fn send_buffinv_command<Q:SerialPort>(openport:&mut Q)->bool{
  let buf:Vec<u8> = vec![0x01];
  let state = send_protocol_data(openport, 0x80,1, buf);
  state
}

pub fn send_reset_command<Q:SerialPort>(openport:&mut Q)->bool{
  // let buf:Vec<u8> = vec![0xA0,0x03,0x00,0x70,0xED];
  let buf:Vec<u8> = Vec::new();
  let state = send_protocol_data(openport, 0x70,0, buf);
  state
}    

pub fn send_setpower_command<Q:SerialPort>(openport:&mut Q,power:u8)->bool{

  let buf:Vec<u8> = vec![power];
  let state = send_protocol_data(openport, 0x76,1, buf);
  state

}

pub fn send_getpower_command<Q:SerialPort>(openport:&mut Q)->bool{

  let buf:Vec<u8> = Vec::new();
  let state = send_protocol_data(openport, 0x77,0, buf);
  state

}


fn transform_u32_to_array_of_u8(x:u32) -> Vec<u8>{
    let b1 : u8 = ((x >> 24) & 0xff) as u8;
    let b2 : u8 = ((x >> 16) & 0xff) as u8;
    let b3 : u8 = ((x >> 8) & 0xff) as u8;
    let b4 : u8 = (x & 0xff) as u8;
    return vec![b1,b2,b3,b4]
}

pub fn send_setcustomfreq_command<Q:SerialPort>(openport:&mut Q,freq:u32)->bool{

  
  let mut temp = transform_u32_to_array_of_u8(freq);
  temp.remove(0);
  let mut buf:Vec<u8> = vec![0x04,0x00,0x19,0x01];
  buf.append(&mut temp);
  let state = send_protocol_data(openport, 0x78,buf.len() as u8, buf);
  state

}


pub fn send_setstandardfreq_command<Q:SerialPort>(openport:&mut Q,standard:u8,start:u8,end:u8)->bool{

  let buf:Vec<u8> = vec![standard,start,end];
  let state = send_protocol_data(openport, 0x78,3, buf);
  state

}

pub fn send_getstandardfreq_command<Q:SerialPort>(openport:&mut Q)->bool{

  let buf:Vec<u8> = Vec::new();
  let state = send_protocol_data(openport, 0x79,0, buf);
  state

}

pub fn send_setrflink_command<Q:SerialPort>(openport:&mut Q,mode:u8)->bool{

  let buf:Vec<u8> = vec![mode];
  let state = send_protocol_data(openport, 0x69,1, buf);
  state

}

pub fn send_getrflink_command<Q:SerialPort>(openport:&mut Q)->bool{

  let buf:Vec<u8> = Vec::new();
  let state = send_protocol_data(openport, 0x6A,0, buf);
  state

}

pub fn send_realtimeinv_command<Q:SerialPort>(openport:&mut Q)->bool{

  let buf:Vec<u8> = vec![0x01];
  let state = send_protocol_data(openport, 0x89,1, buf);
  state

}

pub fn send_opencw_command<Q:SerialPort>(openport:&mut Q)->bool{

  let buf:Vec<u8> = vec![0x01];
  let state = send_protocol_data(openport, 0x3E,1, buf);
  state

}

pub fn send_match_epc_command<Q:SerialPort>(openport:&mut Q,mode:u8,mut epc:Vec<u8>)->bool{

  let mut buf:Vec<u8> = Vec::new();
  buf.push(mode);
  if mode == 0x00{
    let len:u8 = epc.len() as u8;
    buf.push(len);
    buf.append(&mut epc);
  }
  else {
    buf.push(0);
  }
  let state = send_protocol_data(openport, 0x85,buf.len() as u8, buf);
  state

}

pub fn send_single_read_command<Q:SerialPort>(openport:&mut Q,mut addr:Vec<u8>,membank:u8,mut wordcount:Vec<u8>,mut password:Vec<u8>)->bool{

  let mut buf:Vec<u8> = Vec::new();

  buf.push(membank);
  buf.append(&mut addr);
  buf.append(&mut wordcount);
  buf.append(&mut password);
  let state = send_protocol_data(openport, 0x81,buf.len() as u8, buf);
  state

}

pub fn send_custom_inventory_command<Q:SerialPort>(openport:&mut Q,round:u8,mode:u8,len:u8)->bool{
  let buf:Vec<u8> = vec![0x01,round,mode,len];
  let state = send_protocol_data(openport, 0x8A,buf.len() as u8, buf);
  state
}

pub fn send_set_select_command<Q:SerialPort>(openport:&mut Q,mut epc:Vec<u8>)->bool{
  let len = (epc.len() * 8) as u8;
  let mut buf:Vec<u8> = vec![0x01,0x41,0x00,0x00,0x00,0x20,len,0x01];
  buf.append(&mut epc);  
  let state = send_protocol_data(openport, 0x8D,buf.len() as u8, buf);
  state
}

pub fn send_set_session_command<Q:SerialPort>(openport:&mut Q)->bool{
  let buf:Vec<u8> = vec![0x02,0x00];
  let state = send_protocol_data(openport, 0x5B,buf.len() as u8, buf);
  state
}




pub trait BufFunc {
   fn new(&mut self);
   fn get_single_pack<T:UI>(&mut self,ui:&T)->Result<Vec<u8>,()>; 
   fn decode_inv_epc_data<Q:SerialPort>(&mut self,openport:&mut Q)->bool;
}

pub struct Buf{
  pub data_que:VecDeque<u8>,
  pub pack:Vec<u8>,
  pub pack_len:u8
}


impl BufFunc for Buf {
  fn new(&mut self){
    self.pack = Vec::new();
    self.data_que = VecDeque::new();
    self.pack_len = 0;
   }

  fn get_single_pack<T:UI>(&mut self,ui:&T)->Result<Vec<u8>,()>{

    let mut data_vec:Vec<u8> = Vec::new();

        // let t:String = self.data_que
        // .clone()
        // .iter()
        // .map(|x| convert_to_hex(*x))
        // .collect();
        // ui.output(&("first:".to_owned()+&t));

      let temp = self.data_que.pop_front();
      if temp == Some(0xA0){
        self.pack.clear();
        self.pack.push(temp.unwrap());
        let a = self.data_que.pop_front();
        if a == None{
          return Err(());
        }
        self.pack.push(a.unwrap());
      }
      else {
        if temp == None{
          return Err(());
        }         
        self.pack.push(temp.unwrap());
      }


      let len = self.pack[1] + 2;
      if len > self.pack.len() as u8{
        loop {
          let a = self.data_que.pop_front();
          if a == None{
            break;
          }
          self.pack.push(a.unwrap());

          if len == self.pack.len() as u8{
            break;
          }
        }
      }
      // let t:String = self.pack
      // .clone()
      // .iter()
      // .map(|x| convert_to_hex(*x))
      // .collect();
      // ui.output(&t);

      if self.pack.len() - 2 < self.pack[1].into(){
        return Err(());
      }

        if self.pack[0] == 0xA0{
            let check = self.pack.pop().unwrap();
            let temp = self.pack.clone();
            if check == 0x00{
              return Err(());
            }
            let sum = checksun(temp);
            if check == sum{
              let cmd = self.pack[3];
              if cmd == 0x89{
                if self.pack[1] == 0x4{
                  self.pack.clear();
                  return Err(());
                }
                else {
                    let len = (self.pack[5] >> 3) * 2; 
                    for i in 0..len{
                      data_vec.push(self.pack[(i + 7) as usize]);
                    }

                }
              }
              else if cmd == 0x59{
                if self.pack[1] == 0x4{
                  data_vec.push(self.pack[4]);
                }
                else {

                    let len = (self.pack[5] >> 3) * 2; 
                    data_vec.push(len);
                    for i in 0..len{
                      data_vec.push(self.pack[(i + 7) as usize]);
                    }
                    data_vec.push(self.pack[(len + 9) as usize]);
                    data_vec.push(self.pack[(len + 10) as usize]);
                    data_vec.push(self.pack[(len + 11) as usize]);

                }
              }
              else if cmd == 0x8A {
                 if self.pack[1] == 0x4{
                  if self.pack[4] != 0x10{
                    ui.output("没有读到标签");
                  }
                  return Err(());
                }                 
              }
              else {
                  ui.output("命令无效");
                  return  Err(());
              }
          }
          else {
              ui.output("校验错误");
              return Err(());
          }
        }
        else {
            ui.output("协议头错误");
            return Err(());
        }

      Ok(data_vec)
      

  }

  fn decode_inv_epc_data<Q:SerialPort>(&mut self,openport:&mut Q)->bool{

    let buf = get_uart_data(openport);

    if buf.is_ok(){
      let mut que:VecDeque<u8> = buf.unwrap().into();
      self.data_que.append(&mut que);
      return true;
    }
    return  false;
  }

}





pub fn decode_protocol_data<Q:SerialPort>(openport:&mut Q)->Result<Vec<u8>,()>{

  let mut data_vec:Vec<u8> = Vec::new();

  let buf = get_uart_data(openport);

  if buf.is_ok(){
    let mut data_que:VecDeque<u8> = buf.unwrap().into();
    let mut data:Vec<u8> = Vec::new();
    loop {
      let temp = data_que.pop_front();
      if temp == None{
        break;
      } 

      if temp == Some(0xA0){
        data.clear();
        // let len = data_que[0];
        data.push(temp.unwrap());
        loop {
           let a = data_que.pop_front(); 
           if a == None{
            break;
           }
          data.push(a.unwrap());
        }
        if data[0] == 0xA0{

          // let data_len = data[1];
            let check = data.pop().unwrap();
            if check == checksun(data.clone()){
              let cmd = data[3];
              if cmd == 0x70{//restart
                data_vec.push(data[4]);
              }
              else if cmd == 0x76{//set power 
                data_vec.push(data[4]);
              }
              else if cmd == 0x77{//get power 
                data_vec.push(data[4]);
              }
              else if cmd == 0x78{//set freq
                data_vec.push(data[4]);
              }
              else if cmd == 0x79{//get freq
                data_vec.push(data[4]);
                data_vec.push(data[5]);
                data_vec.push(data[6]);
              }
              else if cmd == 0x69{//set rflink
                data_vec.push(data[4]);
              }
              else if cmd == 0x6A{//get rflink
                data_vec.push(data[4]);
              }
              else if cmd == 0x80{//buf inventory
                if data[1] == 0x4{
                  data_vec.push(data[4]);
                }
                else{
                  data_vec.push(data[4]);
                  data_vec.push(data[5]);
                }
              }

              else if cmd == 0x90{//get epc buff
                let len = data[4] - 4;
                for i in 0..len{
                  data_vec.push(data[(i+7) as usize]);
                }
              }
              else if cmd == 0x93{//clean epc buff
                data_vec.push(data[4]);
              }
              else if cmd == 0x8C{//stop inv
                data_vec.push(data[4]);
              }
              else if cmd == 0x82{// write
                if data[1] == 0x4{
                  data_vec.push(data[4]);
                }
                else {
                  let len = data.len() as u8;
                  for i in 4 ..len{
                    data_vec.push(data[i as usize]);
                  }
                }
              }
              else if cmd == 0x5B {
                data_vec.push(data[4]);
              }
              else if cmd ==  0x3E{
                data_vec.push(data[4]);
              }
              else if cmd == 0x85{
                data_vec.push(data[4]);
              }
              else if cmd == 0x81 {
                data_vec.push(data[4]);
              }
              else if cmd == 0x8D {
                data_vec.push(data[4]);
              }
              else{
                return Err(());
              }
          }
          else {

              return Err(());
          }
        }
        else {
            return Err(());
        }
      }
    }

  }
  else {
    return Err(());
      
  }

  Ok(data_vec)
}


pub fn get_uart_data<Q:SerialPort>(openport:&mut Q)->Result<Vec<u8>,()>{
      let mut count = 0;
      let mut buf:[u8;255] = [0;255];
      loop {
        let read_len = openport.read(&mut buf);
        if read_len.is_ok(){
          return Ok(buf[..read_len.unwrap()].to_vec())
        }
        else if count >= 10 {
            return Err(())
        }         
        else{
          count += 1;
        }
      }

}

pub fn send_protocol_data<Q:SerialPort>(openport:&mut Q,cmd:u8,len:u8,mut data:Vec<u8>)->bool{

  let mut buf:Vec<u8> = vec![0xA0,0x03+len,0x00];

  buf.insert(3, cmd);

  buf.append(&mut data);

  let checksun = checksun(buf.clone());
  buf.insert((len +4).into(), checksun);


  let write = openport.write(&buf[..]);

  if write.is_ok(){
     return true;
  }
  else {
    return false;
  }
}

fn checksun(data:Vec<u8>)->u8{

  let mut checksun:u8 = 0;
  for d in data{
    checksun = checksun.wrapping_add(d);
  }
  checksun = (!checksun ) + 1 ;

  checksun
}

pub fn get_epc_vec(path:String) -> Result<Vec<String>, Error> {
    
    let mut excel: Xlsx<_> = open_workbook(path).unwrap();
    let mut epc_vec:Vec<String> = Vec::new();
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
      for row in r.rows() {
          let log:String = row[1].to_string();
          epc_vec.push(log);
      }

      Ok(epc_vec)
    }
    else {
        Err(From::from("expected at least one record but got none"))
    }
}

pub fn convert_to_hex(d:u8)->String{
  let mut hex = String::new();
  let table = vec!['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F'];
  if d < 16{
    hex.push('0');  
    hex.insert(1, table[d as usize]);
  }
  else {
      let temp = d / 16; 
      let temp1 = d % 16;
      hex.insert(0, table[temp as usize]);
      hex.insert(1, table[temp1 as usize]);
  }
  hex
}



