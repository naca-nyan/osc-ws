<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const serverAddr = ref("ws://");
const parameter = ref("/avatar/parameters/hoge");
const value = ref("1");
const type = ref("Int");

const state: { [key: string]: any } = ref({});

const isDebug = ref(false);

async function send() {
  const body = {
    addr: parameter.value,
    value: value.value,
    typ: type.value,
  };
  await invoke("send_osc_message", body);
}

async function recieveLoop() {
  while (isDebug.value) {
    const [key, value] = await invoke("receive");
    state.value[key] = value;
  }
}

function toggleDebug() {
  isDebug.value = !isDebug.value;
  if (isDebug.value) recieveLoop();
}
</script>

<template>
  <main>
    <h1>OSC for VRChat</h1>
    <label for="parameter">parameter</label>
    <input v-model="parameter" />
    <label for="type">type</label>
    <select name="type" id="type" v-model="type">
      <option>Int</option>
      <option>Bool</option>
      <option>Float</option>
    </select>
    <label for="value">value</label>
    <input v-model="value" />
    <button @click="send()">send</button>
    <button @click="toggleDebug()">
      {{ isDebug ? "disable" : "enable" }} Debug
    </button>
    <table v-if="isDebug">
      <tr v-for="(value, key) in state">
        <td>{{ key }}</td>
        <td>{{ value }}</td>
      </tr>
    </table>
  </main>
</template>

<style>
#app {
  margin-top: 60px;
}

table {
  background-color: #24292f;
}

table td {
  width: 50vh;
}
</style>
