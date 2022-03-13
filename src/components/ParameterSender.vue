<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

import { AvatarParameterConfig, Parameter } from "../avatarconfig";
import ParameterAdder from "./ParameterAdder.vue";

const avatarconfig = ref<AvatarParameterConfig | null>(null);
const editableParameters = ref<Parameter[]>([]);
const parameters = computed(() => {
  if (avatarconfig.value === null) {
    return editableParameters.value;
  }
  return avatarconfig.value.parameters
    .map((p) => p.input)
    .filter((input): input is Parameter => input !== undefined);
});

const emit = defineEmits<{
  (e: "onsend", param: Parameter, value: string): void;
}>();

function addParameter(param: Parameter) {
  editableParameters.value.push(param);
  save();
}

function removeParameter(i: number) {
  if (editableParameters.value[i] === undefined) return;
  editableParameters.value = editableParameters.value.filter((_, j) => i !== j);
  save();
}

function save() {
  const config = JSON.stringify(editableParameters.value, null, 2);
  invoke("save_avatar_config", { config })
    .then(console.log)
    .catch((e) => console.warn(e));
}

function load() {
  invoke("load_avatar_config")
    .then((text) => {
      editableParameters.value = JSON.parse(text as string) as Parameter[];
    })
    .catch((e) => console.warn(e));
}

function send(param: Parameter, value: string) {
  emit("onsend", param, value);
}

async function getParameters(): Promise<object> {
  const avatar = await invoke("get_state", { key: "/avatar/change" });
  if (!avatar) throw new Error("avatar not detected");
  const [_typ, avatarId] = avatar as [string, string];
  const contents = await invoke("read_avatar_config", { avatarId });
  const json = contents as string;
  const trim = json.trim();
  const data = JSON.parse(trim);
  return data;
}

getParameters()
  .then((data) => (avatarconfig.value = data as AvatarParameterConfig))
  .catch((_) => load());
</script>

<template>
  <div class="col-lg-6">
    <div v-if="avatarconfig === null">
      <ParameterAdder @add="addParameter" />
      <div class="mt-3 text-center fst-italic">
        Avatar config not loaded. Try reset-avatar and F5.
      </div>
      <hr />
    </div>
    <div v-else class="text-center mb-3">
      <h5>Avatar: {{ avatarconfig.name }}</h5>
    </div>
    <div v-for="(p, i) in parameters" class="mb-3">
      <div class="row mb-1">
        <div class="col form-label">
          {{ p.address }}
          <span class="badge rounded-pill bg-secondary text-light">
            {{ p.type }}
          </span>
        </div>
        <button
          v-if="avatarconfig === null"
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
  </div>
</template>
