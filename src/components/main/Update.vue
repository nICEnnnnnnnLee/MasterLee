<script setup>
import { ref } from "vue";
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater'
import { relaunch } from '@tauri-apps/api/process'

const tips_update = ref("");

async function update() {
  try {
    tips_update.value = "查询更新信息..."
    const { shouldUpdate, manifest } = await checkUpdate()
    console.log(shouldUpdate, manifest);
    if (shouldUpdate) {
      tips_update.value = "安装更新中..."
      // display dialog
      await installUpdate()
      // install complete, restart the app
      tips_update.value = "重启中..."
      await relaunch()
    } else {
      tips_update.value = "不需要更新"
    }
  } catch (error) {
    console.log(error)
    tips_update.value = JSON.stringify(error);
  }
}
</script>


<template>
  <div class="row">
    <div class="btn">
      <button type="button" @click="update()">检查更新</button>
      <span class="tips">{{ tips_update }}</span>
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