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
  //TODO: this is a terrible way of doing the date
  let date = jot.time_modified.toString().split('T')[0];
</script>



<div class="row">
  <div class="box">
    <div class="action-buttons">
      <div class="date-label">{date}</div>
      <div>
        <button class="action-button">c</button>
        <button class="action-button">e</button>
      </div>

    </div>
    <div class="text-area">{@html marked(jot.text)}</div>
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
    margin-top: 10px;
    cursor: pointer;
  }
  .action-button {
      top: 0em;
      right: 0em;
      display: inline-block;
      padding: 0px 6px;
      border-radius: 50px;
      background-color: #383838;
      color: #757474;
      font-size: 20px;
      font-weight: bold;
      text-align: center;
      cursor: pointer;
      align-content: end;
  }
  .text-area{
    /* background-color: #7574741a; */
    border-radius: 10px;
    margin-top: 0.1em;
    margin-bottom: 0.1em;
    padding: 0.5em;
    word-wrap: break-word;
    white-space: pre-line;
    text-align: left;
  }
  .action-buttons {
    display: flex;
    
    justify-content: space-between;
  }
  .date-label {
    color: #757474;
    text-align: left;
    margin: 0em;
    font-weight: bold;
  }
</style>