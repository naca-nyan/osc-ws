<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import { Parameter } from "../avatarconfig";
import ParameterAdder from "./ParameterAdder.vue";

const parameters = ref<Parameter[]>([]);

const emit = defineEmits<{
  (e: "onsend", param: Parameter, value: string): void;
}>();

function addParameter(param: Parameter) {
  parameters.value.push(param);
  save();
}

function removeParameter(i: number) {
  if (parameters.value[i] === undefined) return;
  parameters.value = parameters.value.filter((_, j) => i !== j);
  save();
}

function save() {
  const config = JSON.stringify(parameters.value, null, 2);
  invoke("save_config", { config })
    .then(console.log)
    .catch((e) => console.warn(e));
}

function load() {
  invoke("load_config")
    .then((text) => {
      parameters.value = JSON.parse(text as string) as Parameter[];
    })
    .catch((e) => console.warn(e));
}

function send(param: Parameter, value: string) {
  emit("onsend", param, value);
}

load();
</script>

<template>
  <ParameterAdder @add="addParameter" />
  <hr />
  <div v-for="(p, i) in parameters" class="mb-3">
    <div class="row mb-1">
      <div class="col form-label">
        {{ p.address }}
        <span class="badge rounded-pill bg-secondary text-light">
          {{ p.type }}
        </span>
      </div>
      <button
        @click="removeParameter(i)"
        class="btn-close pe-4"
        aria-label="Remove"
      ></button>
    </div>
    <div class="row">
      <div v-if="p.type === 'Int'" class="btn-group">
        <button
          v-for="v in ['0', '1', '2', '3', '4', '5', '6']"
          @click="send(p, v)"
          class="btn btn-outline-primary"
        >
          {{ v }}
        </button>
      </div>
      <div v-else-if="p.type === 'Bool'" class="btn-group">
        <button @click="send(p, 'true')" class="btn btn-outline-primary">
          True
        </button>
        <button @click="send(p, 'false')" class="btn btn-outline-primary">
          False
        </button>
      </div>
      <div v-else-if="p.type === 'Float'" class="btn-group">
        <button
          v-for="v in [
            '0.0',
            '0.1',
            '0.2',
            '0.3',
            '0.4',
            '0.5',
            '0.6',
            '0.7',
            '0.8',
            '0.9',
            '1.0',
          ]"
          @click="send(p, v)"
          class="btn btn-outline-primary"
        >
          {{ v }}
        </button>
      </div>
    </div>
    <hr />
  </div>
</template>
