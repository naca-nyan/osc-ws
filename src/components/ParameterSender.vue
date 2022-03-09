<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

type Typ = "Int" | "Bool" | "Float";

interface Parameter {
  addr: string;
  typ: Typ;
}

const parameters = ref<Parameter[]>([]);

const addrBase = ref("/avatar/parameters/");
const paramDefault = "VRCEmotes";
const parameter = ref("");
const typ = ref("Int");

function addParameter() {
  const actualParam = parameter.value ? parameter.value : paramDefault;
  const param = {
    addr: addrBase.value + actualParam,
    typ: typ.value as Typ,
  };
  parameters.value.push(param);
  const config = JSON.stringify(parameters.value, null, 2);
  invoke("write_avatar_config", { config }).then(console.log);
}

async function send(param: Parameter, value: string) {
  const body = { ...param, value };
  console.log(body);
  await invoke("send_osc_message", body);
}

invoke("read_avatar_config")
  .then((text) => {
    parameters.value = JSON.parse(text as string) as Parameter[];
  })
  .catch((e) => console.warn(e));
</script>

<template>
  <div class="mt-4 col-lg-6">
    <div class="row">
      <div class="col-8">
        <label class="visually-hidden">Parameter</label>
        <div class="input-group">
          <input v-model="addrBase" class="input-group-text" />
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
      <button @click="addParameter()" class="btn btn-primary col-2">Add</button>
    </div>
    <hr />
    <div v-for="p in parameters" class="mb-3">
      <div class="row mb-1">
        <div class="col form-label">
          {{ p.addr }}
          <span class="badge rounded-pill bg-secondary text-light">
            {{ p.typ }}
          </span>
        </div>
      </div>
      <div class="row">
        <div v-if="p.typ === 'Int'" class="btn-group">
          <button
            v-for="i in ['0', '1', '2', '3', '4', '5', '6']"
            @click="send(p, i)"
            class="btn btn-outline-primary"
          >
            {{ i }}
          </button>
        </div>
        <div v-else-if="p.typ === 'Bool'" class="btn-group">
          <button @click="send(p, 'true')" class="btn btn-outline-primary">
            True
          </button>
          <button @click="send(p, 'false')" class="btn btn-outline-primary">
            False
          </button>
        </div>
        <div v-else-if="p.typ === 'Float'" class="input-group">
          <input type="number" value="1.0" step="0.1" class="form-control" />
          <button class="btn btn-outline-primary col-3" disabled>
            (not implemented)
          </button>
        </div>
      </div>
      <hr />
    </div>
  </div>
</template>
