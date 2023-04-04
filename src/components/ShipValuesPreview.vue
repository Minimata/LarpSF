<script setup lang="ts">
import { ref } from "vue";
import { initializeApp } from "firebase/app";
import { getFirestore, collection, doc } from "firebase/firestore";
import { useCollection, useDocument } from "vuefire";

const firebaseApp = initializeApp({
  apiKey: import.meta.env.FIREBASE_API_KEY,
  authDomain: "larp-sf.firebaseapp.com",
  projectId: "larp-sf",
  storageBucket: "larp-sf.appspot.com",
  messagingSenderId: import.meta.env.FIREBASE_MESSAGING_SENDER_ID,
  appId: import.meta.env.FIREBASE_APP_ID,
  measurementId: import.meta.env.FIREBASE_MEASUREMENT_ID,
});

// used for the firestore refs
const db = getFirestore(firebaseApp);

// here we can export reusable database references
const shipDoc = useDocument(doc(collection(db, "ship"), "ship"));

const value = ref(0);

const buttonClicked = () => {
  value.value++;
};
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
      Add to values
    </button>

    <div
      class="p-1 row-start-2 col-span-2 w-full h-fit grid grid-cols-1 grid-rows-3 gap-1"
    >
      <progress
        class="progress progress-accent w-full"
        :value="shipDoc!.energy * 100"
        max="100"
      ></progress>
      <progress
        class="progress progress-success w-full"
        :value="shipDoc!.food * 100"
        max="100"
      ></progress>
      <progress
        class="progress progress-primary w-full"
        :value="shipDoc!.oxygen * 100"
        max="100"
      ></progress>
    </div>
  </div>
</template>
