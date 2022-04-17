import logo from './logo.svg';
import './App.css';
import { invoke } from '@tauri-apps/api/tauri'
import { emit, listen } from '@tauri-apps/api/event'
import { appWindow, WebviewWindow } from '@tauri-apps/api/window'
import { useEffect } from 'react';
function App() {



  useEffect(async () => {
    //监听事件前端(前端使用emit('click', {}))和后端事件(后端使用app.emit_all或app.emit_to)
    const unlisten = await listen('click', event => {
      alert("event");
      console.log('click');
      // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
      // event.payload is the payload object
    })
    //
    const unlisten2 = await listen('_error', event => {
      alert("event");
      console.log('click');
      // event.event is the event name (useful if you want to use a single callback fn for multiple event types)
      // event.payload is the payload object
    })

    //系统事件监听(本例是窗口缩放事件)
    appWindow.listen('tauri://resize', ({ event, payload }) => {
      // payload here is a `PhysicalPosition`
      console.log("系统特殊事件");
    })
    // emits the `click` event with the object payload


  }, []);
  const onClick = () => {
    // invoke('my_custom_command_param', { invoke_message: 'Hello!' });
    //有参调用
    invoke('my_custom_command_param', { invokeMessage: 'Hello rust!' });
    //无参调用
    invoke('my_custom_command');
    //返回值调用
    invoke('my_custom_command_return').then((message) => alert(message));
    //返回值为result类型调用
    invoke('my_custom_command_result')
      .then((message) => console.log(message))
      .catch((error) => console.error(error));

    //异步调用
    invoke('my_custom_command_async').then(() => console.log('Completed!'));
    //可以传递windows实例
    invoke('my_custom_command_windows');
    //热键注册
    invoke('my_custom_command_handle');
    //全局状态
    invoke('my_custom_command_state');
    //发出事件
    emit('click', {
      theMessage: 'Tauri is awesome!'
    });
    //向后端发起全局事件
    appWindow.emit('event-name', { message: 'Tauri is awesome!' });
    //测试后端向前端发起事件
    invoke('my_custom_command_emit');


    // create a new webview window and emit an event only to that window
    // const webview = new WebviewWindow('window')
    // webview.emit('event')
  }



  return (
    <div className="App">
      <h2>React-rust-Demo</h2>
      <div className='input-event'>
        <label htmlFor="">标签:</label>
        <input type="text" />
        <button onClick={onClick}>点击</button>
        <p><strong>说明:</strong>调用后端函数 </p>
      </div>

    </div>
  );
}

export default App;
