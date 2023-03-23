<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { marked } from 'marked';
  import EditJotPopup from './EditJotPopup.svelte';
  
  let showJotPopup = false;
  export let search_jots: () => {};
  export let jot: {id: Number, text: String, img_path: String, time_create: String, time_modified: String};
  export let new_jot_id;
  
  let tags: {id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}[] = [];
  async function get_all_tags_for_jot() {
    tags = await invoke("get_all_tags_for_jot", {id: jot.id});
  }
  get_all_tags_for_jot().catch(() => console.log("failed to get jot tags"));

  //Open if it is the new jot
  if (new_jot_id === jot.id) {
    new_jot_id = Number;
    showJotPopup = true;
  }
  
  function editJot() {
    showJotPopup = true;
    document.body.style.overflow = 'hidden';
  }

</script>

<div class="row">
  <button class="box" on:click={editJot}>
    <svg class="edit-icon" xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000">
      <path d="M0 0h24v24H0V0z" fill="none"/><path d="M3 17.46v3.04c0 .28.22.5.5.5h3.04c.13 0 .26-.05.35-.15L17.81 9.94l-3.75-3.75L3.15 17.1c-.1.1-.15.22-.15.36zM20.71 7.04c.39-.39.39-1.02 0-1.41l-2.34-2.34c-.39-.39-1.02-.39-1.41 0l-1.83 1.83 3.75 3.75 1.83-1.83z"/>
    </svg>
    <div class="text-area">{@html marked(jot.text)}</div>
    {#each tags as tag, i}
      <div class="tag">
        <p>{tag.title}</p>
      </div>
    {/each}     
  </button>
</div>
{#await get_all_tags_for_jot()}
{:then}
<EditJotPopup jotId={jot.id} bind:visible={showJotPopup} bind:tags={tags} bind:jot={jot} bind:search_jots={search_jots}/>
{/await}

<style>
  .edit-icon {
    text-align: left;
    position: absolute;
    right: 5px;
    top: 5px;
    fill: var(--primary-color);
  }
</style>