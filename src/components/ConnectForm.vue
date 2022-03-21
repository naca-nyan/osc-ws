<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";

const readyStates = ["CONNECTING", "OPEN", "CLOSING", "CLOSED"] as const;
type ReadyState = typeof readyStates[number];

const emit = defineEmits<{
  (e: "onmessage", message: MessageEvent<any>): void;
  (e: "onclose"): void;
}>();

const user = ref("");
const room = ref("");

const serverAddr = "wss://osc-ws.herokuapp.com";

async function load() {
  const res = await invoke("load_config");
  const config = JSON.parse(res as string);
  user.value = config.user ?? "";
  room.value = config.room ?? "";
}

async function save() {
  const res = await invoke("load_config").catch(() => "{}");
  const config = JSON.parse(res as string);
  config.user = user.value;
  config.room = room.value;
  const json = JSON.stringify(config, null, 2);
  await invoke("save_config", { config: json });
}

load().catch(console.warn);

let sock: null | WebSocket = null;
const state = ref<ReadyState>("CLOSED");
function connect() {
  state.value = "CONNECTING";
  try {
    if (!user.value) throw new Error("おなまえがないよ～");
    if (!room.value) throw new Error("あいことばがないよ～");
    const url = new URL(serverAddr);
    const params = new URLSearchParams({ user: user.value, room: room.value });
    url.search = params.toString();
    sock = new WebSocket(url);
    sock.onopen = (_) => {
      state.value = "OPEN";
      save();
    };
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
  sock.onclose = () => {
    state.value = "CLOSED";
    emit("onclose");
    sock = null;
  };
  sock.close();
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
      <label class="visually-hidden">Server Address</label>
      <div class="input-group">
        <input
          v-model="user"
          class="form-control"
          :disabled="state !== 'CLOSED'"
          placeholder="おなまえ"
        />
        <input
          v-model="room"
          class="form-control"
          :disabled="state !== 'CLOSED'"
          placeholder="あいことば"
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
        <button v-else-if="state === 'CLOSING'" class="btn btn-danger" disabled>
          Disconnecting...
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
