<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref, computed, reactive, watchEffect } from "vue";
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

const text = ref("");
const typing = ref(false);
watchEffect(() => {
  const t = text.value.length > 0;
  if (typing.value != t) typing.value = t;
});
watchEffect(async () => {
  const value = typing.value;
  await invoke("send_osc_message", {
    addr: "/chatbox/typing",
    value: value.toString(),
    typ: "Bool",
  });
});
async function send() {
  const immediate = true;
  await invoke("send_osc_message", {
    addr: "/chatbox/input",
    value: text.value,
    typ: `String ${immediate}`,
  });
  text.value = "";
}
</script>
<template>
  <header class="container-fluid p-3 mb-3 bg-primary text-white text-center">
    <h1>ChatBox for VRChat</h1>
  </header>
  <main class="container">
    <div class="row">
      <div class="col-lg-6">
        <div class="input-group mb-3">
          <input
            v-model="text"
            @keydown.enter="send"
            type="text"
            class="form-control"
            placeholder="chat message"
            aria-label="chat message"
          />
          <button @click="send" class="btn btn-outline-secondary">Send</button>
        </div>
      </div>
    </div>
  </main>
</template>

<style>
@import "bootstrap";
@import "bootstrap-icons";
</style>
