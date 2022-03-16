<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
import { Parameter } from "./avatarconfig";

import ParameterReceiver from "./components/ParameterReceiver.vue";
import ParameterSender from "./components/ParameterSender.vue";
import ConnectForm from "./components/ConnectForm.vue";
import ParameterSenderAuto from "./components/ParameterSenderAuto.vue";

const routes = ["Auto detect parameters", "Manually add parameters"] as const;
type Routes = typeof routes[number];

const route = ref<Routes>(routes[0]);

// ref for WebSockerAdderssBar
const ws = ref();

async function onmessage(message: MessageEvent) {
  console.log(message);
  const { addr, typ, value } = await JSON.parse(message.data);
  await invoke("send_osc_message", { addr, typ, value });
}

async function onsend(param: Parameter, value: string) {
  const { address: addr, type: typ } = param;
  const body = { addr, typ, value };
  if (ws.value.state === "OPEN") {
    ws.value.send(body);
  } else {
    await invoke("send_osc_message", body);
  }
}
</script>
<template>
  <header class="container-fluid p-3 mb-3 bg-primary text-white text-center">
    <h1>Parameter Sync for VRChat</h1>
    <ConnectForm @onmessage="onmessage" ref="ws" />
  </header>
  <main class="container">
    <div class="row">
      <div class="col-lg-6">
        <ul class="nav nav-tabs mb-3">
          <li v-for="r in routes" class="nav-item">
            <a
              @click="route = r"
              :class="r === route ? 'nav-link active' : 'nav-link'"
            >
              {{ r }}
            </a>
          </li>
        </ul>
        <div v-if="route === 'Auto detect parameters'">
          <ParameterSenderAuto @onsend="onsend" />
        </div>
        <div v-if="route === 'Manually add parameters'">
          <ParameterSender @onsend="onsend" />
        </div>
      </div>
      <div class="col-lg-6">
        <ParameterReceiver />
      </div>
    </div>
  </main>
</template>

<style>
@import "bootstrap";
</style>
