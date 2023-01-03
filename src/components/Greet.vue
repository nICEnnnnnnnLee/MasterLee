<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { resolveResource } from '@tauri-apps/api/path'
// alternatively, use `window.__TAURI__.path.resolveResource`
import { readTextFile } from '@tauri-apps/api/fs'
// alternatively, use `window.__TAURI__.fs.readTextFile`

import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'


const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

async function readConf() {
  // `config.default.json` is the value specified on `tauri.conf.json > tauri > bundle > resources`
  const resourcePath = await resolveResource('resources/config/config.default.json');
  const conf = JSON.parse(await readTextFile(resourcePath));
  console.log(conf);
  greetMsg.value = "read using js: " + conf.foo;
}

async function readConfFromRust() {
  greetMsg.value = "read using rust: " + await invoke("read_conf");
}

async function update() {
  try {
    const { shouldUpdate, manifest } = await checkUpdate()
    console.log(shouldUpdate, manifest);
    if (shouldUpdate) {
      // display dialog
      await installUpdate()
      // install complete, restart the app
      await relaunch()
    }
  } catch (error) {
    console.log(error)
  }
}
</script>

<style>
.btn {
  margin-top: 5px;
}
</style>
<template>
  <div class="card">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="button" @click="greet()">Greet</button>
  </div>
  <div class="btn">
    <button type="button" @click="readConf()">ReadConf</button>
    <button type="button" @click="readConfFromRust()">ReadConf2</button>
    <button type="button" @click="update()">检查更新</button>
  </div>

  <p>{{ greetMsg }}</p>
</template>
