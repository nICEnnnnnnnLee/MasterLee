<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const tips_pull_doh_servers_and_set_resolvers = ref("当前尚未设置doh");
async function pull_doh_servers_and_set_resolvers() {
  tips_pull_doh_servers_and_set_resolvers.value = "进行中..."
  invoke("pull_doh_servers_and_set_resolvers").then(() => {
    tips_pull_doh_servers_and_set_resolvers.value = "已完成..."
  }).catch((err) => {
    console.log(err);
    tips_pull_doh_servers_and_set_resolvers.value = "出现异常"
  })
}
</script>


<template>
  <div class="row">
    <div class="btn">
      <button type="button" @click="pull_doh_servers_and_set_resolvers()">查找可用的DoH</button>
      <span class="tips">{{ tips_pull_doh_servers_and_set_resolvers }}</span>
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