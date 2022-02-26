<script setup lang="ts">
import { ref } from "vue";
import { invoke } from '@tauri-apps/api/tauri'

const serverAddr = ref("ws://");
const parameter = ref("/avatar/parameters/hoge");
const value = ref("1");
const type = ref("Int");

async function send() {
  const body = {
    addr: parameter.value,
    value: value.value,
    typ: type.value,
  };
  await invoke("send_osc_message", body);
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
  </main>
</template>

<style>
#app {
  margin-top: 60px;
}
</style>
