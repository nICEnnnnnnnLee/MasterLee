<script setup>
import { ref } from "vue";

import Proxy from "./main/Proxy.vue";
import Update from "./main/Update.vue";
import Host from "./main/Host.vue";
import DoH from "./main/DoH.vue";
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
octocaptcha.com
raw.githubusercontent.com
repository-images.githubusercontent.com
uploads.github.com
user-images.githubusercontent.com`

let _domains = (localStorage && localStorage.getItem('_domains')) || defaultDomains;
let _bind_ip = (localStorage && localStorage.getItem('_bind_ip')) || "127.0.0.1";
let _bind_port = (localStorage && localStorage.getItem('_bind_port')) || 443;
_bind_port = Number(_bind_port)

const bind_ip = ref(_bind_ip);
const bind_port = ref(_bind_port);
const domains_str = ref(_domains)

</script>


<template>
  <div class="card">
    <p>Protected Domains </p>
    <textarea id="domains-input" rows="10" cols="50" v-model="domains_str"></textarea>
  </div>
  <div class="card">
    <p>
      监听:
      <input id="ip-input" style="width: 80px;" v-model="bind_ip" placeholder="请输入监听ip..." />
      :
      <input id="port-input" type="number" style="width: 50px;" v-model="bind_port" placeholder="port" />
    </p>
  </div>
  <DoH/>
  <Host :domains_str="domains_str" />
  <Proxy :bind_ip="bind_ip" :bind_port="bind_port"  />
  <Update/>
</template>

<style scoped>
</style>