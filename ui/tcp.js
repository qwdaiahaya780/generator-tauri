toastr.options.positionClass = 'toast-center-center';
toastr.options.timeOut = 1000;

async function sendTcp() {
  var ip = $("#ip").val();
  var port = $("#port").val();
  var content = $("#content").val();
// 字符集
  var charset = $("#charset").val();
  if (ip == null || ip.trim() == "") {
    toastr.warning('请输入IP!');
    return false;
  }
  if (port == null || port.trim() == "") {
    toastr.warning('请输入端口!');
    return false;
  }
  if (content == null || content.trim() == "") {
    toastr.warning('请输入内容!');
    return false;
  }

  // const response = await window.__TAURI__.invoke('send_tcp', ip, parseInt(port), content);
  console.log("开始调用send_tcp:ip=" + ip + ",port=" + port + ",content=" + content + ",charset=" + charset);
  const { invoke } = window.__TAURI__.tauri
      invoke('send_tcp', { ip: ip, port: parseInt(port), content: content, charset: charset})
        .then((rs) => {
          $("#response").val(rs);
        })
  
}



