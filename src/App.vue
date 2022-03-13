<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { reactive, ref } from "vue";

import ParameterReceiver from "./components/ParameterReceiver.vue";
import ParameterSender from "./components/ParameterSender.vue";
import { Parameter } from "./parameter";

const serverAddr = ref("");
const serverAddrDefault = "wss://";

const readyStates = ["CONNECTING", "OPEN", "CLOSING", "CLOSED"] as const;
type ReadyState = typeof readyStates[number];

class MyWebSocket {
  sock?: WebSocket;
  state: ReadyState = "CLOSED";
  onmessage: ((this: WebSocket, ev: MessageEvent) => any) | null = null;
  connect(url: string, protocols?: string) {
    this.state = "CONNECTING";
    this.sock = new WebSocket(url, protocols);
    this.sock.onerror = (err) => {
      addLogs(err.toString());
      this.state = "CLOSED";
    };
    this.sock.onopen = () => {
      this.state = "OPEN";
    };
    this.sock.onmessage = this.onmessage;
  }
  send(body: object) {
    if (this.sock === undefined) return;
    this.sock.send(JSON.stringify(body));
  }
  close() {
    if (this.sock === undefined) return;
    this.state = "CLOSING";
    this.sock.close();
    this.sock.onclose = () => {
      this.state = "CLOSED";
      this.sock = undefined;
    };
  }
}

const sock = reactive(new MyWebSocket());

sock.onmessage = async (message) => {
  console.log(message);
  const { addr, typ, value } = await JSON.parse(message.data);
  invoke("send_osc_message", { addr, typ, value });
};

function connect() {
  const url = serverAddr.value;
  try {
    sock.connect(url);
  } catch (e: any) {
    console.error(e);
    addLogs(url ? "Invalid url: No URL supplied" : e.toString());
  }
}

const logs = ref<string[]>([]);

function addLogs(log: string) {
  logs.value.push(log);
  setTimeout(() => {
    logs.value.length = 0;
  }, 2000);
}

async function onsend(param: Parameter, value: string) {
  const body = { ...param, value };
  if (sock.state === "OPEN") {
    sock.send(body);
  } else {
    invoke("send_osc_message", body);
  }
}
</script>
<template>
  <header class="container-fluid p-3 mb-4 bg-primary text-white text-center">
    <h1>Parameter Sync for VRChat</h1>
    <div class="row">
      <div class="col-sm-8 m-auto mt-2">
        <label class="visually-hidden">Server Address</label>
        <div class="input-group">
          <input
            v-model="serverAddr"
            class="form-control"
            :placeholder="serverAddrDefault"
          />
          <button
            v-if="sock.state === 'CLOSED'"
            @click="connect()"
            class="btn btn-info"
          >
            Connect
          </button>
          <button
            v-else-if="sock.state === 'CONNECTING'"
            class="btn btn-info"
            disabled
          >
            Conecting...
          </button>
          <button v-else @click="sock.close()" class="btn btn-danger">
            Disconnect
          </button>
        </div>
      </div>
    </div>
    <div v-if="logs.length" class="mt-3">
      <div class="alert alert-danger alert-dismissable fade show p-1 m-auto">
        <div v-for="log in logs">{{ log }}</div>
      </div>
    </div>
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
