<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, computed, reactive } from "vue";
import { Parameter, ParameterInfo } from "./avatarconfig";
import { Client } from "./client";

import ParameterReceiver from "./components/ParameterReceiver.vue";
import ConnectForm from "./components/ConnectForm.vue";
import ParameterSender from "./components/ParameterSender.vue";
import ParameterSyncSettings from "./components/ParameterSyncSettings.vue";

const routes = ["Sync Settings"] as const;
type Routes = typeof routes[number] | number;

const route = ref<Routes>(routes[0]);

// ref for WebSocket
const ws = ref();

const clients = ref<Client[]>([]);

let syncedNames: string[] = [];

const infoMapDefault = new Map<Number, ParameterInfo[]>();
const infoMap = reactive(infoMapDefault);
const parameters = computed<ParameterInfo[]>(() => {
  if (typeof route.value !== "number") return [];
  const id = route.value as number;
  return infoMap.get(id) ?? [];
});

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
      const { id, body } = event;
      switch (body.on) {
        case "sync":
          const { params } = body;
          infoMap.set(id, params);
          break;
        case "send":
          const { param } = body;
          await invoke("send_osc_message", param);
          break;
      }
      break;
  }
}

function onclose() {
  clients.value.length = 0;
  route.value = routes[0];
}

function onSyncedParameterChange(params: ParameterInfo[]) {
  console.log(syncedNames);
  if (!ws.value) return;
  if (ws.value.state !== "OPEN") return;
  const body = {
    on: "sync",
    params: params,
  };
  ws.value.send(body);
}

async function onsend(param: Parameter, value: string) {
  const { address: addr, type: typ } = param;
  const body = {
    on: "send",
    param: { addr, typ, value },
  };
  if (ws.value.state === "OPEN") {
    ws.value.send(body);
  } else {
    await invoke("send_osc_message", body.param);
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
          <li v-for="{ id, user } in clients" class="nav-item">
            <a
              @click="route = id"
              :class="route === id ? 'nav-link active' : 'nav-link'"
            >
              <i class="bi bi-circle-fill text-success pe-1"></i>
              <span>{{ user }}</span>
            </a>
          </li>
          <li v-for="r in routes" class="nav-item">
            <a
              @click="route = r"
              :class="r === route ? 'nav-link active' : 'nav-link'"
            >
              {{ r }}
            </a>
          </li>
        </ul>
        <div v-if="typeof route === 'number'">
          <ParameterSender :parameters="parameters" @onsend="onsend" />
        </div>
        <div v-if="route === 'Sync Settings'">
          <ParameterSyncSettings
            :syncedNames="syncedNames"
            @onsend="onsend"
            @onchange="onSyncedParameterChange"
            @onunmounted="(s) => (syncedNames = s)"
          />
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
@import "bootstrap-icons";
</style>
