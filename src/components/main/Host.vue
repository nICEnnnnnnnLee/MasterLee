<script setup>
import { ref, defineProps } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const tips_query_dns_and_set_host = ref("当前host为空");
// const domains_str = ref(_domains)
const {domains_str} = defineProps(['domains_str']);

async function query_dns_and_set_host() {
  tips_query_dns_and_set_host.value = "进行中..."
  let domains = domains_str.split("\n").map(val => val.trim()).filter(val => val != "" && !val.startsWith("#"))
  console.log(domains_str);
  console.log(domains);
  localStorage && localStorage.setItem('_domains', domains_str)
  invoke("query_dns_and_set_host", { domains }).then(() => {
    tips_query_dns_and_set_host.value = "已完成..."
  }).catch((err) => {
    console.log(err);
    tips_query_dns_and_set_host.value = "出现异常"
  })
}

</script>


<template>
  <div class="row">
    <div class="btn">
      <button type="button" @click="query_dns_and_set_host()">查询可用的IP</button>
      <span class="tips">{{ tips_query_dns_and_set_host }}</span>
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