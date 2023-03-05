<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { marked } from 'marked';
  import EditJotPopup from './EditJotPopup.svelte';
  import { focusTrap } from 'svelte-focus-trap';
  
  let showJotPopup = false;
  let focusText = true;
  export let search_jots;
  export let jot = {id: Number, text: String, img_path: String, time_create: String, time_modified: String };
  export let new_jot_id;

  let tags = [{id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}];
  async function get_all_tags_for_jot() {
    tags = await invoke("get_all_tags_for_jot", {id: jot.id});
  }
  get_all_tags_for_jot().catch(() => console.log("failed to get jot tags"));

  //Open if it is the new jot
  if (new_jot_id === jot.id) {
    new_jot_id = Number;
    showJotPopup = true;
  }

  //TODO: this is a terrible way of doing the date
  let date = jot.time_modified.toString().split('T')[0];
  
  function editText() {
    focusText = true;
    showJotPopup = true;
  }
  function editTags() {
    focusText = false;
    showJotPopup = true;
  }
</script>

<div class="row">
  <div class="box">
    <div class="action-buttons">
      <div class="date-label">{date}</div>
      <div>
        <button class="action-button" on:click={editText}>e</button>
      </div>
    </div>
    <div class="text-area">{@html marked(jot.text)}</div>
    {#each tags as tag, i}
      <div class="tag">{tag.title}</div>
    {/each}
    <!-- TODO: Remove inability focus add tags?-->
    <button tabIndex="-1" class="add-tag" on:click={editTags}>+</button>
  </div>
</div>
{#await get_all_tags_for_jot()}
{:then}
<EditJotPopup jotId={jot.id} bind:visible={showJotPopup} bind:tags={tags} bind:jot={jot} bind:search_jots={search_jots} bind:focusText={focusText}/>
{/await}