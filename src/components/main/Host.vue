<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { resolveResource } from '@tauri-apps/api/path'
import { readTextFile, writeTextFile, createDir } from '@tauri-apps/api/fs'


const tips = ref("当前host为空");
let propss = defineProps(['domains_str', 'timeout']);
const init = () => {
  return new Promise(function (resolve, reject) {
    readConf().then((conf) => {
      invoke('add_hosts', { conf })
        .then(() => {
          tips.value = "已加载配置"
          resolve()
        })
        .catch((e) => reject(e))
    }).catch((e) => reject(e))
  })
}

defineExpose({
  init
})
async function readConf() {
  const dir = await resolveResource('data')
  const path = await resolveResource('data/host.txt')
  await createDir(dir, { recursive: true })
  const conf = await readTextFile(path);
  return conf.split("\n").map(val => val.trim()).filter(val => val != "" && !val.startsWith("#"))
}

async function saveConf(conf) {
  const dir = await resolveResource('data')
  const path = await resolveResource('data/host.txt')
  await createDir(dir, { recursive: true })
  let sortedConf = conf.sort((recordA, recordB) => {
    let domainA = recordA.split(' ')[1]
    let domainB = recordB.split(' ')[1]
    return domainA > domainB ? 1 : domainA == domainB ? 0 : -1

  })
  await writeTextFile(path, sortedConf.join("\n"))
}


async function query_dns_and_set_host() {
  tips.value = "进行中..."
  let domains = propss.domains_str.split("\n").map(val => val.trim()).filter(val => val != "" && !val.startsWith("#"))
  localStorage && localStorage.setItem('_domains', propss.domains_str)
  localStorage && localStorage.setItem('_timeout', propss.timeout)
  invoke("query_dns_and_set_host", { domains, timeout: propss.timeout }).then(() => {
    tips.value = "已完成..."
    invoke('get_all_hosts')
      .then((hosts) => {
        console.log(hosts)
        saveConf(hosts).await
      })
      .catch((e) => console.error(e))
  }).catch((err) => {
    console.log(err);
    tips.value = "出现异常"
  })
}

</script>


<template>
  <div class="row">
    <div class="btn">
      <button type="button" @click="query_dns_and_set_host()">查询可用的IP</button>
      <span class="tips">{{ tips }}</span>
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