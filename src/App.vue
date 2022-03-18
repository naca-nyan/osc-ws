<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
import { Parameter } from "./avatarconfig";
import { Client } from "./client";

import ParameterReceiver from "./components/ParameterReceiver.vue";
import ParameterSender from "./components/ParameterSender.vue";
import ConnectForm from "./components/ConnectForm.vue";
import ParameterSenderAuto from "./components/ParameterSenderAuto.vue";
import ClientList from "./components/ClientList.vue";

const routes = ["Auto detect parameters", "Manually add parameters"] as const;
type Routes = typeof routes[number];

const route = ref<Routes>(routes[0]);

// ref for WebSockerAdderssBar
const ws = ref();

const clients = ref<Client[]>([]);

async function onmessage(message: MessageEvent) {
  const event = JSON.parse(message.data);
  console.log(event);
  switch (event.event) {
    case "updateRoom":
      if (event.roomInfo) clients.value = event.roomInfo;
      break;
    case "closed":
      clients.value = clients.value.filter((c) => c.id !== event.id);
      break;
    case "message":
      await invoke("send_osc_message", event.body);
      break;
  }
}

function onclose() {
  clients.value.length = 0;
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
    <ConnectForm @onmessage="onmessage" @onclose="onclose" ref="ws" />
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
    <div class="row mt-2">
      <div class="col">
        <ClientList :clients="clients" />
      </div>
    </div>
  </main>
</template>

<style>
@import "bootstrap";
@import "bootstrap-icons";
</style>
