<script setup lang="ts">
import { window } from "@tauri-apps/api";

import { computed, ref, watchEffect, onMounted } from "vue";
import { getRandom, setRndMode } from "../random.ts";

import Dice from "./Dice.vue";

const rnd = ref([0]);

const handleRandom = async () => {
  rnd.value = await getRandom();
};

onMounted(() => handleRandom())

const oneValue = computed(() => {
  return rnd.value[rnd.value[Date.now() % rnd.value.length] % rnd.value.length];
});

const fullscreen = ref(true);

watchEffect(() => window.getCurrent().setFullscreen(fullscreen.value));

const toggleFullscreen = () => {
  fullscreen.value = !fullscreen.value;
};

onMounted(() => {
  document.addEventListener('keydown', (ev) => {
    console.log(ev)
    if (ev.code == 'KeyP') toggleFullscreen()
    else if (ev.code == 'Space') handleRandom()
  })
})

const point = computed(
  () => ((oneValue.value % 6) + 1) as 1 | 2 | 3 | 4 | 5 | 6
);
</script>

<template>
  <div class="container">
    <div class="box-1">
      <Dice
        :point="point"
        size="large"
        :theme="{ bgColor: '#fff', itemColor: '#000' }"
        :style="{ fontSize: '80px', zIndex: '99' }"
      />
      <p class="code">{{ JSON.stringify(rnd, null, 2) }}</p>
    </div>
    <div class="box-2">
      <!-- <button @click="toggleFullscreen()">Fullscreen</button> -->
       <p>God plays Dice?</p>
      <button @click="handleRandom()">Random </button>
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  height: 100vh;
  flex-direction: column;

  justify-content: center;
  align-items: center;
}

.box-1 {
  display: flex;
  flex-direction: column;
  height: 50vh;

  justify-content: center;
  align-items: center;
}

.box-2 {
  display: flex;
  flex-direction: column;
  height: 50vh;
  justify-content: center;
  align-items: center;
  font-size: 100px;
  font-family: 'Fira Code'
}

.box-2 > button {
  font-size: 32px;
  padding: 20px;
  background: #ffffff30;
  color: #fff;
  border-radius: 6px;
  border: 1px solid #000
}

.code {
  word-wrap: normal;
  font-size: 16px;
  font-family: "Fira Code";
  line-height: normal;
  color: #ffffff30;
  position: absolute;
  z-index: 0;
}
</style>
