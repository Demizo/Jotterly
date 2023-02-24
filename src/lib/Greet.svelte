<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { marked } from 'marked';
  let name = "";
  let greetMsg = "";
  
  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name });
  }
</script>


{@html marked(greetMsg)}
<div>
  <div class="row">
    <input id="greet-input" inputmode="search" on:keyup={greet} placeholder="Enter a name..." bind:value={name} />
    <button on:click={greet}> Greet </button>
  </div>
  <p>{greetMsg}</p>
  <input id="test-input" placeholder="Jot..." bind:value={name} on:keyup={greet} />
  <textarea bind:value={name}></textarea>
</div>
