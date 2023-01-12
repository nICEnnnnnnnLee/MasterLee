<script setup>
import { onMounted, ref } from "vue";

import Proxy from "./main/Proxy.vue";
import Update from "./main/Update.vue";
import Host from "./main/Host.vue";
import DoH from "./main/DoH.vue";
import { WebviewWindow } from '@tauri-apps/api/window'

let defaultDomains = `assets-cdn.github.com
avatars.githubusercontent.com
avatars0.githubusercontent.com
camo.githubusercontent.com
cloud.githubusercontent.com
codeload.github.com
favicons.githubusercontent.com
gist.github.com
gist.githubusercontent.com
github.com
github.githubassets.com
marketplace-screenshots.githubusercontent.com
objects.githubusercontent.com
octocaptcha.com
raw.githubusercontent.com
repository-images.githubusercontent.com
uploads.github.com
user-images.githubusercontent.com`

let _domains = (localStorage && localStorage.getItem('_domains')) || defaultDomains;
let _bind_ip = (localStorage && localStorage.getItem('_bind_ip')) || "127.0.0.1";
let _bind_port = (localStorage && localStorage.getItem('_bind_port')) || 443;
_bind_port = Number(_bind_port)
let _timeout = (localStorage && localStorage.getItem('_timeout')) || 2;
_timeout = Number(_timeout)

const bind_ip = ref(_bind_ip);
const bind_port = ref(_bind_port);
const timeout = ref(_timeout);
const tips_load_config = ref("");
const domains_str = ref(_domains)
const doh = ref()
const host = ref()
onMounted(() => {
  loadConfig()
})
const loadConfig = async () => {
  tips_load_config.value = "尝试读取配置..."
  try {
    await doh.value.init()
    await host.value.init()
    tips_load_config.value = "配置读取完毕"
  } catch (err) {
    tips_load_config.value = "相关配置不存在"
  }
}


function open_window_host() {
  const webview = new WebviewWindow('dns', {
    url: 'dns.html',
    width: 300,
    height: 500,
    decorations: false,
  })
  webview.once('tauri://created', function () {
    console.log('webview window successfully created');
  })
  webview.once('tauri://error', function (e) {
    console.log('an error occurred during webview window creation', e);
  })
}
</script>


<template>
  <div class="card">
    <!-- <p>Protected Domains </p> -->
    <textarea id="domains-input" rows="15" cols="50" v-model="domains_str"></textarea>
  </div>
  <div class="card">
    <p>
    <div class="btn">
      <button type="button" @click="open_window_host">打开DNS查询页面</button>
    </div>
    </p>
    <p>
      监听:
      <input id="ip-input" style="width: 80px;" v-model="bind_ip" placeholder="请输入监听ip..." />
      :
      <input id="port-input" type="number" style="width: 50px;" v-model="bind_port" placeholder="port" />
    </p>
    <p>
      测试HTTPS连接超时时间:
      <input id="timeout-input" type="number" style="width: 30px;" v-model="timeout" placeholder="超时时间(s)" />
      s
    </p>
  </div>
  <div class="row">
    <div class="btn">
      <button type="button" @click="loadConfig">从配置文件加载</button>
      <span class="tips">{{ tips_load_config }}</span>
    </div>
  </div>
  <DoH ref="doh" />
  <Host ref="host" :domains_str="domains_str" :timeout="timeout" />
  <Proxy :bind_ip="bind_ip" :bind_port="bind_port" />
  <Update />
</template>

<style scoped>
.btn {
  margin-top: 5px;
}

button {
  width: 190px;
}

p {
  margin-top: 7px;
  margin-bottom: 7px;
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