<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const isDebug = ref(false);
const states: { [key: string]: any } = ref({});

function toggleDebug() {
  isDebug.value = !isDebug.value;
  if (isDebug.value) recieveLoop();
}

async function recieveLoop() {
  const receives = (await invoke("get_states")) as [string, string, string][];
  receives.forEach(([addr, typ, value]) => {
    states.value[addr] = [typ, value];
  });
  const refresh_rate = 60;
  setTimeout(() => {
    if (isDebug.value) recieveLoop();
  }, 1000 / refresh_rate);
}
</script>
<template>
  <div class="col-lg-6">
    <div class="table-responsive">
      <table v-if="isDebug" class="table">
        <thead>
          <tr>
            <th>Address</th>
            <th>Type</th>
            <th>Value</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="([typ, value], addr) in states">
            <td>{{ addr }}</td>
            <td>{{ typ }}</td>
            <td>{{ value }}</td>
          </tr>
        </tbody>
      </table>
    </div>
    <button @click="toggleDebug()" class="btn btn-secondary">
      {{ isDebug ? "Hide" : "Show" }} parameter states
    </button>
  </div>
</template>
<style>
td:last-child,
th:last-child {
  width: 35vw;
}
</style>
