<script lang="ts">
import { ref } from 'vue';
import { bootcamp_chat_backend, canisterId, createActor } from '../../declarations/bootcamp_chat_backend';
import { AuthClient } from '@dfinity/auth-client';
import { HttpAgent } from '@dfinity/agent';
import type { Identity } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';

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
      if (!this.identity || this.identity.getPrincipal() === Principal.anonymous()) {
        throw new Error("PLZ log in")
      }
      const backend = createActor(canisterId, {
        agentOptions: {
          identity: this.identity
        }
      });
      await backend.add_note(this.newNote)
      await this.pobierzNotatki()
    },
    async pobierzNotatki() {
      if (!this.identity || this.identity.getPrincipal() === Principal.anonymous()) {
        throw new Error("PLZ log in")
      }
      this.notes = await bootcamp_chat_backend.get_notes(this.identity.getPrincipal())
    },
    async login() {
      const authClient = await AuthClient.create();
      await authClient.login({
        identityProvider: "http://avqkn-guaaa-aaaaa-qaaea-cai.localhost:4943/"
      })

      const identity = authClient.getIdentity();
      console.log("Zalogowano", identity.getPrincipal())
      this.identity = identity;
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
