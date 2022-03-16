<script setup lang="ts">
import { ref } from "vue";

const readyStates = ["CONNECTING", "OPEN", "CLOSING", "CLOSED"] as const;
type ReadyState = typeof readyStates[number];

const emit = defineEmits<{
  (e: "onmessage", message: MessageEvent<any>): void;
}>();

const user = ref("");
const room = ref("");

const serverAddr = ref("");
const serverAddrDefault = "wss://";

let sock: null | WebSocket = null;
const state = ref<ReadyState>("CLOSED");
function connect() {
  state.value = "CONNECTING";
  try {
    if (!user.value) throw new Error("おなまえがないよ～");
    if (!room.value) throw new Error("あいことばがないよ～");
    if (!serverAddr.value) throw new Error("さーばーあどれすがないよ");
    const url = new URL(serverAddr.value);
    const params = new URLSearchParams({ user: user.value, room: room.value });
    url.search = params.toString();
    sock = new WebSocket(url);
    sock.onopen = (_) => (state.value = "OPEN");
    sock.onmessage = (message) => emit("onmessage", message);
    sock.onerror = (err) => {
      state.value = "CLOSED";
      addLogs("Connection failed: " + JSON.stringify(err));
    };
  } catch (e: any) {
    state.value = "CLOSED";
    addLogs(e.toString());
  }
}

function send(body: object) {
  if (state.value !== "OPEN") return;
  if (sock == null) return;
  sock.send(JSON.stringify(body));
}

function close() {
  if (sock == null) return;
  state.value = "CLOSING";
  sock.close();
  sock.onclose = () => {
    state.value = "CLOSED";
    sock = null;
  };
}

const logs = ref<string[]>([]);

function addLogs(log: string) {
  logs.value.push(log);
  setTimeout(() => {
    logs.value.length = 0;
  }, 3000);
}

function onkeydown(e: KeyboardEvent) {
  if (e.ctrlKey || e.metaKey) {
    if (e.key === "Enter") {
      connect();
    }
  }
}

defineExpose({
  state,
  send,
});
</script>
<template>
  <div class="row">
    <div class="col-sm-8 m-auto mt-2">
      <div class="row">
        <div class="col-6 pe-1">
          <input
            v-model="user"
            class="form-control"
            :disabled="state !== 'CLOSED'"
            placeholder="おなまえ"
          />
        </div>
        <div class="col-6 ps-1">
          <input
            v-model="room"
            class="form-control"
            :disabled="state !== 'CLOSED'"
            placeholder="あいことば"
          />
        </div>
      </div>
    </div>
  </div>
  <div class="row">
    <div class="col-sm-8 m-auto mt-2">
      <label class="visually-hidden">Server Address</label>
      <div class="input-group">
        <input
          v-model="serverAddr"
          class="form-control"
          @keydown="onkeydown"
          :disabled="state !== 'CLOSED'"
          :placeholder="serverAddrDefault"
        />
        <button
          v-if="state === 'CLOSED'"
          @click="connect()"
          class="btn btn-info"
        >
          Connect
        </button>
        <button
          v-else-if="state === 'CONNECTING'"
          class="btn btn-info"
          disabled
        >
          Conecting...
        </button>
        <button v-else @click="close()" class="btn btn-danger">
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
</template>
