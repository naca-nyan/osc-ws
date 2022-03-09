<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const isDebug = ref(false);
const state: { [key: string]: any } = ref({});

function toggleDebug() {
  isDebug.value = !isDebug.value;
  if (isDebug.value) recieveLoop();
}

async function recieveLoop() {
  while (isDebug.value) {
    const [key, value] = await invoke("receive");
    state.value[key] = value;
  }
}
</script>
<template>
  <div class="mt-4 col-lg-6">
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
      {{ isDebug ? "Disable" : "Enable" }} debug mode
    </button>
  </div>
</template>
