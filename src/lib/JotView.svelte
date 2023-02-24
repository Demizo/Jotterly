<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { marked } from 'marked';
  import EditTagsPopup from './EditTagsPopup.svelte';

  let showTagPopup = false;

  function displayTagPopup() {
    showTagPopup = true;
  }
  export let jot = {id: Number, text: String, img_path: String, time_create: String, time_modified: String };
  let tags = [{id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}];
  async function get_all_tags_for_jot() {
    tags = await invoke("get_all_tags_for_jot", {id: jot.id});
    console.log(tags);
  }
  get_all_tags_for_jot().catch(() => console.log("failed to get jot tags"));
  
</script>



<div class="row">
  <div class="box">
    <p>{@html marked(jot.text)}</p>
    {#each tags as tag, i}
      <div class="tag">{tag.title}</div>
    {/each}
    <button class="add-tag" on:click={displayTagPopup}>+</button>
    <EditTagsPopup jotId={jot.id} bind:visible={showTagPopup} bind:tags={tags} />
  </div>
</div>
<style>
  .add-tag {
    display: inline-block;
    padding: 0px 6px;
    border-radius: 50px;
    background-color: #383838;
    color: #757474;
    font-size: 20px;
    font-weight: bold;
    text-align: center;
    margin-right: 10px;
    margin-bottom: 10px;
    cursor: pointer;
  }
</style>