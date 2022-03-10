<script setup lang="ts">
import { reactive, ref } from "vue";

import ParameterReceiver from "./components/ParameterReceiver.vue";
import ParameterSender from "./components/ParameterSender.vue";

const serverAddr = ref("");
const serverAddrDefault = "wss://";

const readyStates = ["CONNECTING", "OPEN", "CLOSING", "CLOSED"] as const;
type ReadyState = typeof readyStates[number];

class MyWebSocket {
  sock?: WebSocket;
  state: ReadyState = "CLOSED";
  onmessage: ((this: WebSocket, ev: MessageEvent) => any) | null = null;
  connect(url: string, protocols?: string) {
    this.sock = new WebSocket(url, protocols);
    this.state = "CONNECTING";
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
}

const sock = reactive(new MyWebSocket());

sock.onmessage = (message) => {
  console.log(message);
};

function connect() {
  const url = serverAddr.value;
  try {
    sock.connect(url);
  } catch (e: any) {
    console.error(e);
    addLogs(url ? "Invalid url: No URL supplied" : e.toString());
  }
  console.log(sock.sock?.readyState);
}

const logs = ref<string[]>([]);

function addLogs(log: string) {
  logs.value.push(log);
  setTimeout(() => {
    logs.value.length = 0;
  }, 2000);
}

//TODO: Delete this
serverAddr.value = "ws://localhost:5000";
</script>
<template>
  <header class="container-fluid p-3 bg-primary text-white text-center">
    <h1>OSC for VRChat</h1>
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
          <button v-else class="btn btn-info" disabled>Connected</button>
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
      <ParameterSender />
      <ParameterReceiver />
    </div>
  </main>
</template>

<style>
@import "bootstrap";
</style>
