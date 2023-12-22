const { dialog,os ,path,tauri } = window.__TAURI__;

let appwindow;

let comport;

let outputBox;


let ConnectButton;
let DisconnectButton;

let StartMatchButton;
let StopMatchButton;


let matchnum;
let StartInvButton;
let StopInvButton;

let comselection;
let startstate;
let invstate;

let connectuartstate;

let modeflag;
let coutlist;
let couter;
let match_suc_flag;

window.onunload = () =>{
  let uiState = {
    uartsel_his  : comselection.selectedIndex,
    connecting: ConnectButton.style.display !== 'none',
    marchrunning: StartMatchButton.style.display === 'none',
    invrunning: StartInvButton.style.display === 'none',
    writedata_his : matchnum.value,
  };
  let uiJSON = JSON.stringify(uiState);
  sessionStorage.setItem('pageState', uiJSON);
}


window.addEventListener("DOMContentLoaded", async () => {
  ConnectButton = document.getElementById('ConnectButton');
  DisconnectButton = document.getElementById('DisconnectButton');
  StartMatchButton = document.getElementById('openButton');
  StopMatchButton = document.getElementById('closeButton');
  StartInvButton = document.getElementById('startinvButton');
  StopInvButton = document.getElementById('stopinvButton');
  comselection = document.getElementById('uart_com');
  outputBox = document.getElementById('outputBox');
  matchnum = document.getElementById("writenumber_txt");
  appwindow = window.__TAURI__.window.appWindow;

  await appwindow.listen('outputMsg',(event)=>{
    output(event.payload.message);
  });

  await appwindow.listen('enableNextUi',(_event)=>{
    enableNextUI();
  });

  await appwindow.listen('repeate',async (_event)=>{

  });

  await appwindow.listen('enableUi',async (_event)=>{
  enableUI();
  
  var d = new Date();
  var minute = d.getMinutes();
  var second = d.getSeconds();
  var milisec = d.getMilliseconds();
  var time = minute + ":" + second+ ":"+milisec;
  output(time);

    if(modeflag == false){

      modeflag = true;

      if(coutlist >= 1){
          modeflag = false;
          startstate = true;
          startrun_state(startstate);
          let allTables = document.getElementById("epc_table");
          let epc = allTables.rows[couter].cells[1].innerText;
          // E2 00 02 EE 03 CB 01 01 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F 10 11 12 13 14 15 16
          await tauri.invoke('start_match_epc',{epc:epc
                                              });       
          if (match_suc_flag == true){
            match_suc_flag = false;
            coutlist = coutlist - 1;
            couter = couter + 1;
          }
      }


    }

  });

  await appwindow.listen('getinvEPC',(event)=>{
    add_port_epc(event.payload.message);
  });

  await appwindow.listen('setMatchEPC',(event)=>{
    
    if(modeflag == true){
      let len = matchnum.value;
      add_match_epc(len,event.payload.message);
    }
    else{

      add_match_epc(couter,event.payload.message);

    }

  });

  let uiState = JSON.parse(sessionStorage.getItem('pageState'));
  if(uiState){

    if(uiState.connecting){
      disableNextUI();
      connectuartstate = false;
      connect_state(connectuartstate);
    }
    else{
      connectuartstate = true;
      enableNextUI();
      connect_state(connectuartstate);
      if(uiState.invrunning){
        invstate = true;
        startinv_state(invstate);
      } 
      else{
        if(uiState.marchrunning){
          startstate = true;
          startrun_state(startstate);
        }
      }
    }
    comselection.selectedIndex = uiState.uartsel_his; 
    matchnum.value = uiState.writedata_his;

  }
  else{
    disableNextUI();
  }



});


function output(msg) {
  outputBox.innerText += '\n' + msg;
  outputBox.scrollTop = outputBox.scrollHeight;
}

function add_com_selection(com){

  var x = false;
  var t = "";
  var i;
  for(i = 0;i < comselection.length;i++){
    t = comselection.options[i].text;
    if(com == t){
      x = true;
    }
  }

  if(!x){
    var opt = document.createElement('option');
    opt.text = com;
    comselection.add(opt);
  }
  else{

  }
}

let connectuart = async (button) =>{

  if(button == 'connect'){

    var x = comselection.selectedIndex;
    if(x > 0){
      comport = comselection.options[x].text;
      await tauri.invoke('connect_uart',{comport:comport,});
 
    }
    else{
      output("请先选择串口号 然后再连接");
    }
    
  }
  else{
    connectuartstate = false;
    connect_state(connectuartstate);

    output("断开串口连接");

    disableNextUI();
  }

}

let setbuttonstate = (state) =>{

  if(state === "start"){
      StartMatchButton.style.display = 'none';
      StopMatchButton.style.display = '';
  }
  else{
      StartMatchButton.style.display = '';
      StopMatchButton.style.display = 'none';
  }
}

let setbutton1state = (state) =>{
  if(state == "start"){
    StartInvButton.style.display = 'none';
    StopInvButton.style.display = '';
  }
  else{
    StartInvButton.style.display = '';
    StopInvButton.style.display = 'none';
  }
}

let connect_state = (state) => {
  if(state){
    DisconnectButton.style.display = '';
    ConnectButton.style.display = 'none';
  }
  else{
    DisconnectButton.style.display = 'none';
    ConnectButton.style.display = '';
  }
}

let startrun = async(button)=>{

  var d = new Date();
  var minute = d.getMinutes();
  var second = d.getSeconds();
  var milisec = d.getMilliseconds();
  var time = minute + ":" + second+ ":"+milisec;
  output(time);
  match_suc_flag = false;

  if(button == 'start'){
     
    let len = matchnum.value;
    if(len > 0){
      modeflag = true;
      output("单个匹配模式");
      let allTables = document.getElementById("epc_table");
      let num = allTables.rows.length;
      if(num > 1){

        if(num > len){

          startstate = true;
          startrun_state(startstate);
          let epc = allTables.rows[len].cells[1].innerText;
          // E2 00 02 EE 03 CB 01 01 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F 10 11 12 13 14 15 16
          await tauri.invoke('start_match_epc',{epc:epc
                                              });
        }
        else{

          output("没有这么多端口号");
        }
      }
      else{
        output("请先盘存端口号再匹配");
      }
    }
    else{
      // 遍历模式
      output("遍历模式");
      let allTables = document.getElementById("epc_table");
      let num = allTables.rows.length;
      if(num > 1){
          coutlist = num - 1;
          modeflag = false;
          startstate = true;
          startrun_state(startstate);
          couter = 1;
          let epc = allTables.rows[1].cells[1].innerText;
          // E2 00 02 EE 03 CB 01 01 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F 10 11 12 13 14 15 16
          await tauri.invoke('start_match_epc',{epc:epc
                                              });

      }
      else{
        output("请先盘存端口号再匹配");
      }
        // output("匹配端口不能为0");
    }


  }
  else{
    modeflag = true;
    startstate = false;
    startrun_state(startstate);
    await tauri.invoke('stop_match');
  }
}

let startinv = async(button)=>{
  modeflag = true;
  if(button == 'start'){
    invstate =true;
    startinv_state(invstate);
    await tauri.invoke('inventory_epc');
  }
  else{
    invstate = false;
    startinv_state(invstate);
    await tauri.invoke('stop_inventory');
  }

}




let enableNextUI = async () =>{
  connectuartstate = true;
  connect_state(connectuartstate);
  comselection.disabled = true;
  document.getElementById('startinvButton').disabled = false;
  document.getElementById('openButton').disabled = false;
  document.getElementById('writenumber_txt').disabled = false;
}

let disableNextUI = async () =>{
  comselection.disabled = false;
  StartMatchButton.disabled = true;
  StartInvButton.disabled = true;
  document.getElementById('startinvButton').disabled = true;
  document.getElementById('openButton').disabled = true;
  document.getElementById('writenumber_txt').disabled = true;
}

let enableUI = async () =>{

  setbuttonstate("stop");
  DisconnectButton.disabled = false;
  StartMatchButton.disabled = false;
  StartInvButton.disabled = false;
  matchnum.disabled = false;
}

let disableUI = async () =>{
  DisconnectButton.disabled = true;
  StartMatchButton.disabled = true;
  StartInvButton.disabled = true;
  matchnum.disabled = true;

}

let startrun_state = (state) =>{
  if(state){
      setbuttonstate("start");
      // setcolor_state1('run');
      disableUI();
  }
  else{
    setbuttonstate("stop");
    // setcolor_state1('wait');
     enableUI();
  }
}

let startinv_state = (state) => {
  if(state){
    setbutton1state("start");
    disableUI();

  }
  else{
    setbutton1state("stop");
    enableUI();

  }
}


let add_port_epc=(epc)=>{
  let allTables = document.getElementById("epc_table");
  let num = allTables.rows.length;
  let flag = "false";
  for(var i = 0;i < num;i++){
       let value = allTables.rows[i].cells[1].innerText;
       if(value == epc){
        flag = "true";
       }
  }

  if(flag == "false"){

    let newRow = allTables.insertRow();
    let PortID = newRow.insertCell();
    let PortNum = newRow.insertCell();
    let Node = newRow.insertCell();
    PortID.innerHTML = num;
    PortNum.innerHTML = epc;
    Node.innerHTML = "-";
  }

}

let add_match_epc=(num,epc)=>{
  let allTables = document.getElementById("epc_table");

  let num1 = allTables.rows.length;
  let flag = "false";
  for(var i = 0;i < num1;i++){
       let value = allTables.rows[i].cells[2].innerText;
       if(value == epc){
        flag = "true";
       }
  }

  if(flag == "false"){
    allTables.rows[num].cells[2].innerText = epc;
    match_suc_flag = true;
  }

}


window.connectuart = connectuart;
window.startrun = startrun;
window.startinv = startinv;