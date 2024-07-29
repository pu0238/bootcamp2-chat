<script lang="ts">
import { ref } from 'vue';
import { bootcamp_chat_backend } from '../../declarations/bootcamp_chat_backend';
import { AuthClient } from '@dfinity/auth-client';
import { HttpAgent } from '@dfinity/agent';
import type { Identity } from '@dfinity/agent';

export default {
  data() {
    return {
      newNote: "",
      notes: [] as string[],
      identity: undefined as undefined | Identity,
    }
  },
  methods: {
    async dodajNotatke() {
      await bootcamp_chat_backend.add_note(this.newNote)
      await this.pobierzNotatki()
    },
    async pobierzNotatki() {
      this.notes = await bootcamp_chat_backend.get_notes()
    },
    async login() {
      const authClient = await AuthClient.create();
      await authClient.login({
        identityProvider: "http://avqkn-guaaa-aaaaa-qaaea-cai.localhost:4943/"
      })

      const identity = authClient.getIdentity();
      console.log("Zalogowano", identity.getPrincipal())
      this.identity = identity;
      
     //const agent = new HttpAgent({ identity });
    }
  },
  mounted(){
    this.pobierzNotatki()
  }
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    {{ identity?.getPrincipal() }} <button @click="login">login</button>
    <div>
      {{ notes }}
    </div>
    <div>
      <textarea v-model="newNote"></textarea><button @click="dodajNotatke">Dodaj notatke</button>
    </div>
  </main>
</template>
