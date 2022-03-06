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
  <header class="container-fluid p-5 bg-primary text-white text-center">
    <h1>OSC for VRChat</h1>
  </header>
  <main class="container">
    <div class="row">
      <div class="mt-4 col-sm-6">
        <form>
          <div class="mb-3">
            <label for="parameter" class="form-label">parameter</label>
            <input v-model="parameter" class="form-control" />
          </div>
          <div class="mb-3">
            <label for="type" class="form-label">type</label>
            <select name="type" id="type" v-model="type" class="form-select">
              <option>Int</option>
              <option>Bool</option>
              <option>Float</option>
            </select>
          </div>
          <div class="mb-3">
            <label for="value" class="form-label">value</label>
            <input v-model="value" class="form-control" />
          </div>
          <button @click="send()" class="btn btn-primary">send</button>
        </form>
      </div>
      <div class="mt-4 col-sm-6">
        <table v-if="isDebug" class="table">
          <thead>
            <tr>
              <th>Address</th>
              <th>Value</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(value, key) in state">
              <td>{{ key }}</td>
              <td>{{ value }}</td>
            </tr>
          </tbody>
        </table>
        <button @click="toggleDebug()" class="btn btn-secondary">
          {{ isDebug ? "disable" : "enable" }} Debug
        </button>
      </div>
    </div>
  </main>
</template>

<style>
@import "bootstrap";
</style>
