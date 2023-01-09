<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { resolveResource } from '@tauri-apps/api/path'
import { readTextFile, writeTextFile, createDir } from '@tauri-apps/api/fs'

const tips = ref("当前尚未设置doh");
const init = () => {
  return new Promise(function (resolve, reject) {
    readConf().then((conf) => {
      invoke('add_doh_servers', { conf })
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
  const path = await resolveResource('data/doh.txt')
  await createDir(dir, { recursive: true })
  const conf = await readTextFile(path);
  return conf.split("\n").map(val => val.trim()).filter(val => val != "" && !val.startsWith("#"))
}

async function saveConf(conf) {
  const dir = await resolveResource('data')
  const path = await resolveResource('data/doh.txt')
  await createDir(dir, { recursive: true })
  let sortedConf = conf.sort((a, b) => a > b ? 1 : a == b ? 0 : -1)
  await writeTextFile(path, sortedConf.join("\n"))
}

async function pull_doh_servers_and_set_resolvers() {
  tips.value = "进行中..."
  invoke("pull_doh_servers_and_set_resolvers").then(() => {
    tips.value = "已完成..."
    invoke('get_all_doh_servers')
      .then((dohs) => {
        console.log(dohs)
        saveConf(dohs).await
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
      <button type="button" @click="pull_doh_servers_and_set_resolvers()">查找可用的DoH</button>
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