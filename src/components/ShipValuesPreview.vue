<script setup lang="ts">
import { initializeApp } from "firebase/app";
import { getFirestore, collection, doc, setDoc } from "firebase/firestore";
import { useCollection, useDocument } from "vuefire";

import { invoke } from "@tauri-apps/api/tauri";
import { emit, listen } from "@tauri-apps/api/event";
import { appWindow } from "@tauri-apps/api/window";

const firebaseApp = initializeApp({
  apiKey: import.meta.env.FIREBASE_API_KEY,
  authDomain: "larp-sf.firebaseapp.com",
  projectId: "larp-sf",
  storageBucket: "larp-sf.appspot.com",
  messagingSenderId: import.meta.env.FIREBASE_MESSAGING_SENDER_ID,
  appId: import.meta.env.FIREBASE_APP_ID,
  measurementId: import.meta.env.FIREBASE_MEASUREMENT_ID,
});

interface Ship {
  energy: number;
  energy_growth: number;
  food: number;
  food_growth: number;
  oxygen: number;
  oxygen_growth: number;
}

// used for the firestore refs
const db = getFirestore(firebaseApp);
// here we can export reusable database references
const shipFireDoc = doc(collection(db, "ship"), "ship");
const shipDoc = useDocument(shipFireDoc);

const buttonClicked = async () => {
  console.log(shipDoc);

  const shipValues = {
    energy: shipDoc?.value?.energy,
    energy_growth: shipDoc?.value?.energy_growth,
    food: shipDoc?.value?.food,
    food_growth: shipDoc?.value?.food_growth,
    oxygen: shipDoc?.value?.oxygen,
    oxygen_growth: shipDoc?.value?.oxygen_growth,
  };
  const result = await invoke("game_loop", {
    window: appWindow,
    shipValues: shipValues,
  });
};

const unlistenProgress = appWindow.listen("ShipProgress", ({ payload }) => {
  const shipValues = payload as Ship;
  setDoc(shipFireDoc, shipValues);
  shipDoc!.value! = shipValues;
});
</script>

<template>
  <div class="w-full p-3 grid grid-cols-2 grid-rows-2 gap-2">
    <div class="row-start-1 col-start-1 justify-self-end self-center">
      Ship preview
    </div>
    <button
      class="btn btn-outline btn-secondary row-start-1 col-start-2 justify-self-start self-center"
      @click="buttonClicked"
    >
      Start simulation
    </button>

    <div
      class="p-1 row-start-2 col-span-2 w-full h-fit grid grid-cols-1 grid-rows-3 gap-1"
    >
      <progress
        class="progress progress-accent w-full"
        :value="shipDoc?.energy * 100"
        max="100"
      ></progress>
      <progress
        class="progress progress-success w-full"
        :value="shipDoc?.food * 100"
        max="100"
      ></progress>
      <progress
        class="progress progress-primary w-full"
        :value="shipDoc?.oxygen * 100"
        max="100"
      ></progress>
    </div>
  </div>
</template>
