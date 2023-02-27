<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { marked } from 'marked';
  import EditTagsPopup from './EditTagsPopup.svelte';

  let showTagPopup = false;
  
  export let jot = {id: Number, text: String, img_path: String, time_create: String, time_modified: String };

  let tags = [{id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}];
  async function get_all_tags_for_jot() {
    tags = await invoke("get_all_tags_for_jot", {id: jot.id});
    console.log(tags);
  }
  get_all_tags_for_jot().catch(() => console.log("failed to get jot tags"));
  //TODO: this is a terrible way of doing the date
  let date = jot.time_modified.toString().split('T')[0];
  
  let isEditing = false;
  let text = jot.text;
  let temptext = text;
  let actionButtons;
  function toggleEditing() {
    temptext = text;
    if (isEditing) {
      isEditing = false;
      zIndex(0);
    } else {
      isEditing = true;
      zIndex(100);
    }
  }
  function zIndex(index: Number){
    actionButtons.style.zIndex = index;
  }
  function saveChanges() {
    zIndex(0);
    text = temptext;
    isEditing = false;
    save_jot_text();
  }
  async function save_jot_text() {
    await invoke("update_jot_text", {id: jot.id, text: text, img_path: jot.img_path});
  }
  function calcTextAreaHeight(event) {
    const textarea = event.target;
    textarea.style.height = "auto";
    textarea.style.height = `${textarea.scrollHeight}px`;
  }
  function displayTagPopup() {
    showTagPopup = true;
  }
</script>



<div class="row">
  <div class="box">
    <div class="action-buttons">
      <div class="date-label">{date}</div>
      <div bind:this={actionButtons}>
        {#if !isEditing}
        <button class="action-button">c</button>
        <button class="action-button" on:click={toggleEditing}>e</button>
        {:else}
        <button class="action-button">d</button>
        <button disabled={temptext.toString().trim().length <= 0 || temptext === text} class="action-button" on:click={saveChanges}>s</button>
        <button class="action-button" on:click={toggleEditing}>x</button>
        {/if}
      </div>
    
    </div>
    {#if !isEditing}
      <div class="text-area">{@html marked(text)}</div>
    {:else}
      <textarea style="z-index: 999; position: relative;" autofocus on:select={calcTextAreaHeight} on:input={calcTextAreaHeight} bind:value={temptext}></textarea>
    {/if}
    {#each tags as tag, i}
      <div class="tag">{tag.title}</div>
    {/each}
    <button class="add-tag" on:click={displayTagPopup}>+</button>
  </div>
</div>
<EditTagsPopup jotId={jot.id} bind:visible={showTagPopup} bind:tags={tags} />
{#if isEditing}
<div class="overlay"></div>
{/if}
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
  textarea {
    border-radius: 10px;
    background-color: #7574741a;
    width: 28.7em;
    height: auto;
    font-size: 16px;
    color: white;
    border: 0em;
    padding: 10px;
    resize: none;
    overflow: hidden;
  }

  .overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 1;
  }
</style>