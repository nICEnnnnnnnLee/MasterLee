<script setup>
import { ref, reactive } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const tips = ref("");
const domain = ref("");
const timeout = ref(2);
const testType = ref("tls");

const dnsResults = reactive([])
async function query_dns_of_single_domain() {
  if (domain.value) {
    tips.value = "进行中..."
    dnsResults.value = []
    invoke("query_dns_of_single_domain", { domain: domain.value, timeout: timeout.value, testType: testType.value }).then((data) => {
      tips.value = ""
      dnsResults.value = data.sort((a, b) => a.cost - b.cost)
      console.log(dnsResults.value.length);
    }).catch((err) => {
      console.log(err);
      tips.value = err
    })
  } else {
    tips.value = "domain不能为空"
    dnsResults.value = []
  }
}
</script>

<template>
  <div class="container">
    <div class="row">
      <input id="domain-input" style="width: 180px;" v-model="domain" placeholder="请输入域名..." />
    </div>
    <div class="row">
      <p>
        <span>超时时间：</span>
        <input id="timeout-input" type="number" style="width: 30px;" v-model="timeout" placeholder="超时时间(s)" />
        <span>s</span>
      </p>
    </div>
    <div class="row">
      <p>
        <span>连通性测试：</span>
        <select v-model="testType">
          <option value='tls'>TLS</option>
          <option value='tcp'>TCP</option>
        </select>
      </p>
    </div>
    <div class="row">
      <div class="btn">
        <button type="button" @click="query_dns_of_single_domain()">查询DNS</button>
      </div>
    </div>
    <div class="row mtop" v-if="tips != ''">
      <span class="tips">{{ tips }}</span>
    </div>
    <div class="row mtop" v-if="dnsResults.value && dnsResults.value.length > 0">
      <table width="100%">
        <thead width="100%">
          <tr>
            <th width="70%">IP</th>
            <th width="30%">耗时</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(dnsResult, i) in dnsResults.value" :key="i">
            <td>{{ dnsResult.ip }}</td>
            <td>{{ dnsResult.cost }}ms</td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<style scoped>
.mtop {
  margin-top: 7px;
}

table {
  width: 100%;
  max-width: 512px;
}

p {
  margin-top: 7px;
  margin-bottom: 0px;
}

/* .row {
  height: 40px;
  vertical-align: middle;
} */
</style>
