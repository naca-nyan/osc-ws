<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
import { Parameter } from "./avatarconfig";

import ParameterReceiver from "./components/ParameterReceiver.vue";
import ParameterSender from "./components/ParameterSender.vue";
import WebSocketAddressBar from "./components/WebSocketAddressBar.vue";

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
  <header class="container-fluid p-3 mb-4 bg-primary text-white text-center">
    <h1>Parameter Sync for VRChat</h1>
    <WebSocketAddressBar @onmessage="onmessage" ref="ws" />
  </header>
  <main class="container">
    <div class="row">
      <ParameterSender @onsend="onsend" />
      <ParameterReceiver />
    </div>
  </main>
</template>

<style>
@import "bootstrap";
</style>
