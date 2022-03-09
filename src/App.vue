<script setup lang="ts">
import ParameterReceiver from "./components/ParameterReceiver.vue";
import ParameterSender from "./components/ParameterSender.vue";
</script>
<template>
  <header class="container-fluid p-4 bg-primary text-white text-center">
    <h1>OSC for VRChat</h1>
  </header>
  <main class="container">
    <div class="row">
      <ParameterSender />
      <ParameterReceiver />
    </div>
  </main>
</template>

<style>
@import "bootstrap";
</style>
