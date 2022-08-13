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
const history = ref<[string, Date][]>([]);
async function send() {
  const immediate = true;
  const limit = 144;
  const message = text.value.substring(0, limit);
  await invoke("send_osc_message", {
    addr: "/chatbox/input",
    value: message,
    typ: `String ${immediate}`,
  });
  history.value.push([message, new Date()]);
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
        <div class="chat-history">
          <ul>
            <li v-if="typing">
              <div class="message my-message">
                <span class="dot"></span>
                <span class="dot"></span>
                <span class="dot"></span>
              </div>
            </li>
            <li v-for="[chat, date] in [...history].reverse()">
              <div class="message my-message">
                {{ chat }}
              </div>
              <div class="message message-date">
                {{ date.toLocaleString() }}
              </div>
            </li>
          </ul>
        </div>
      </div>
    </div>
  </main>
</template>

<style>
@import "bootstrap";
@import "bootstrap-icons";

.chat-history ul {
  padding: 0px;
}

.chat-history ul li {
  list-style: none;
  margin-top: 26px;
}
.chat-history .message {
  color: #444;
  padding: 12px 20px;
  line-height: 16px;
  font-size: 16px;
  border-radius: 7px;
  display: inline-block;
  position: relative;
}

.chat-history .my-message {
  background: #efefef;
}

.chat-history .my-message:after {
  bottom: 100%;
  left: 30px;
  border: solid transparent;
  content: " ";
  height: 0;
  width: 0;
  position: absolute;
  pointer-events: none;
  border-bottom-color: #efefef;
  border-width: 10px;
  margin-left: -10px;
}

.chat-history .message-date {
  font-size: 13px;
  color: #bcbcbc;
}

.dot {
  margin: 3px 1px;
  height: 5px;
  width: 5px;
  border-radius: 100%;
  display: inline-block;
  background-color: #b4b5b9;
  animation: 1.2s typing-dot ease-in-out infinite;
}

.dot:nth-of-type(2) {
  animation-delay: 0.15s;
}

.dot:nth-of-type(3) {
  animation-delay: 0.25s;
}

@keyframes typing-dot {
  15% {
    transform: translateY(-35%);
    opacity: 0.5;
  }
  30% {
    transform: translateY(0%);
    opacity: 1;
  }
}
</style>
