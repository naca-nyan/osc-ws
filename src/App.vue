<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

type Typ = "Int" | "Bool" | "Float";

interface Parameter {
  address: string;
  typ: Typ;
}

const parameters = ref<Parameter[]>([]);

const addressBase = ref("/avatar/parameters/");
const paramDefault = "VRCEmotes";
const parameter = ref("");
const typ = ref("Int");

function addParameter() {
  const actualParam = parameter.value ? parameter.value : paramDefault;
  const param = {
    address: addressBase.value + actualParam,
    typ: typ.value as Typ,
  };
  parameters.value.push(param);
}

async function send(param: Parameter, value: string) {
  const body = { value, ...param };
  await invoke("send_osc_message", body);
}

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
  <header class="container-fluid p-5 bg-primary text-white text-center">
    <h1>OSC for VRChat</h1>
  </header>
  <main class="container">
    <div class="row">
      <div class="mt-4 col-lg-6">
        <div class="row">
          <div class="col-8">
            <label class="visually-hidden">Parameter</label>
            <div class="input-group">
              <input v-model="addressBase" class="input-group-text" />
              <input
                v-model="parameter"
                class="form-control"
                placeholder="VRCEmotes"
              />
            </div>
          </div>
          <div class="col-2">
            <label for="type" class="visually-hidden">type</label>
            <select name="type" id="type" v-model="typ" class="form-select">
              <option>Int</option>
              <option>Bool</option>
              <option>Float</option>
            </select>
          </div>
          <button @click="addParameter()" class="btn btn-primary col-2">
            Add
          </button>
        </div>
        <hr />
        <div v-for="p in parameters" class="mb-3 mt-3">
          {{ p }}
        </div>
      </div>
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
    </div>
  </main>
</template>

<style>
@import "bootstrap";
</style>
