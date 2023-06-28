toastr.options.positionClass = 'toast-center-center';
toastr.options.timeOut = 1000;

async function sendTcp() {
  var ip = $("#ip").val();
  var port = parseInt($("#port").val());
  var content = $("#content").val();

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

  const response = await window.__TAURI__.invoke('send_tcp', ip, port, content);
  $("#response").val(response);
}



