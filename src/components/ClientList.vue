<script setup lang="ts">
import { Client } from "../client";

defineProps<{
  clients: Client[];
}>();
</script>
<template>
  <div v-for="{ user } in clients">
    <i class="bi bi-person-circle px-2"></i>
    <span>{{ user }}</span>
  </div>
</template>
