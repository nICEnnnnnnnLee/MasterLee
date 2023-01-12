<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { appWindow, WebviewWindow } from '@tauri-apps/api/window'
// const props = defineProps(['bind_ip', 'bind_port']);
// console.log(props);
const { bind_ip, bind_port } = defineProps(['bind_ip', 'bind_port']);
const proxy_opened = ref(false);
const proxy_btn_text = ref("打开Proxy");
const tips_toggle_proxy = ref("");
const disableBtn = ref(false);

appWindow.listen('proxy-start', (event) => {
  console.log('proxy-start', event.payload);
  if (event.payload === true) {
    tips_toggle_proxy.value = "Proxy已打开..."
    proxy_btn_text.value = "关闭Proxy"
    proxy_opened.value = true
  } else {
    tips_toggle_proxy.value = "请检查端口是否被占用"
  }
})

async function toggle_proxy() {
  tips_toggle_proxy.value = "进行中..."
  disableBtn.value = true
  if (proxy_opened.value) {
    invoke("stop_proxy").then(() => {
      tips_toggle_proxy.value = "Proxy已关闭..."
      proxy_btn_text.value = "打开Proxy"
      proxy_opened.value = false
    }).catch((err) => {
      console.log(err);
      tips_toggle_proxy.value = "出现异常"
    })
  } else {
    localStorage.setItem('_bind_ip', bind_ip)
    localStorage.setItem('_bind_port', bind_port)
    invoke("start_proxy", { addr: bind_ip, port: bind_port }).then(() => {
      // console.log("start_proxy");
    }).catch((err) => {
      console.log(err);
    })
  }
  setTimeout(() => disableBtn.value = false, 2000);
}
</script>
<template>
  <div class="row">
    <div class="btn">
      <button type="button" :disabled=disableBtn @click="toggle_proxy()">{{ proxy_btn_text }}</button>
      <span class="tips">{{ tips_toggle_proxy }}</span>
    </div>
  </div>
</template>

<style scoped>
.btn {
  margin-top: 5px;
}

button {
  width: 190px;
}

.tips {
  overflow-x: hidden;
  overflow-y: hidden;
  white-space: nowrap;
  word-break: keep-all;
  text-overflow: ellipsis;
  margin-left: 10px;
  text-align: start;
  display: inline-block;
  width: 190px;
  max-height: 40px;
}
</style>