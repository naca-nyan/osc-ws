<script setup lang="ts">
import { ref } from "vue";

const serverAddr = ref("ws://");
const parameter = ref("/avatar/parameters/hoge");
const value = ref("1");
const type = ref("Int");

function send() {
  const vrchat_proxy = "http://localhost:9000";
  const headers = { "Content-Type": "application/json" };
  const body = JSON.stringify({
    path: parameter.value,
    value: value.value,
    type: type.value,
  });
  fetch(vrchat_proxy, { method: "POST", headers, body });
}
</script>

<template>
  <main>
    <h1>OSC Sharing for VRChat</h1>
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
    <button @click="send">send</button>
  </main>
</template>

<style>
#app {
  margin-top: 60px;
}
</style>
