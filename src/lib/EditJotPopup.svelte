<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { focusTrap } from 'svelte-focus-trap';
    import { onMount } from 'svelte';
    
    export let visible = false;
    export let jotId: Number;
    export let jot: {id: Number, text: String, img_path: String, time_create: String, time_modified: String };
    export let search_jots: () => {};
    export let tags: {id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}[] = [];
    let temp_tags = tags.slice(0, tags.length);
    let query = "";
    let pending = false;
    let text = jot.text;
    let temptext = text;
    
    function closePopup() {
      query = "";
      temptext = text;
      temp_tags = tags.slice(0, tags.length);
      visible = false;
      document.body.style.overflow = 'auto';
    }
    function handleEscDown(event:  KeyboardEvent) {
      if (visible && event.key === 'Escape') {
        closePopup();
      }
    }
    let tags_list: {id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}[];
    async function search_tags() {
      if (query.trim().length > 2 || query.trim().length == 0 && pending == false) {
        let tag_ids = temp_tags.map(tag => tag.id);
        tags_list = await invoke("search_tags", {query: query, tagIds: tag_ids}).catch(() => console.log("failed to get tags")) as {id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}[];
        tags_list = tags_list.filter(t => !temp_tags.map(tag => tag.title).includes(t.title));
      }
    }
    async function add_new_tag_to_jot() {
      temp_tags.push({id: -1, title: query, color: "", priority: 0, time_create: "", time_modified: ""});
      temp_tags = [...temp_tags];
      query = "";
      search_tags();
    }
    function add_tag(tag: {id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String}){
        temp_tags.push(tag);
        temp_tags = [...temp_tags];
    }
    function remove_tag(title: String){
      temp_tags = temp_tags.filter(t => t.title != title);
    }
    async function save_jot_tags() {
        let new_tags = temp_tags.filter((t) => t.id == -1);
        //filter out new tags
        temp_tags = temp_tags.filter((t) => t.id != -1);
        
        let remove_tags = tags.filter((t) => !temp_tags.map((tag) => tag.title).includes(t.title));
        let add_tags = temp_tags.filter((t) => !tags.map((tag) => tag.title).includes(t.title));

        //remove tags
        for(let i = 0; i < remove_tags.length; i++) {
          await invoke("remove_tag_from_jot", {tagId: remove_tags[i].id, jotId: jotId}).catch(() => console.log("failed to remove tag"));
        }
        
        //add tags
        for(let i = 0; i < add_tags.length; i++) {
          await invoke("add_tag_to_jot", {tagId: add_tags[i].id, jotId: jotId}).catch(() => console.log("failed to add tag"));
        }
        
        //create and add new tags
        for(let i = 0; i < new_tags.length; i++) {
          let tag = await invoke("add_new_tag_to_jot", {title: new_tags[i].title, jotId: jotId}).catch(() => console.log("failed to add new tag")) as {id: Number, title: String, color: String, priority: Number, time_create: String, time_modified: String};
          add_tag(tag);
        }
        
    }
    async function saveChanges() {
      text = temptext;
      await save_jot_tags();
      await save_jot_text();
      jot.text = text;
      tags = temp_tags.slice(0, temp_tags.length);
      await search_tags();
    }
    async function save_jot_text() {
      await invoke("update_jot_text", {id: jot.id, text: text, img_path: jot.img_path});
    }
    async function delete_jot() {
      await invoke("delete_jot", {id: jot.id});
      document.body.style.overflow = 'auto';
      search_jots();
    }
    document.addEventListener('keydown', handleEscDown);
  </script>
  
  {#if visible}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="popup" tabindex="-1" on:mousedown={closePopup}>
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div class="popup-content" use:focusTrap on:mousedown|stopPropagation>
        <div class="action-buttons">
          <button class="action-button" on:click={delete_jot}>
            <svg class="delete-icon" xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="26px" viewBox="0 0 24 24" width="26px" fill="#000000">
              <g><path d="M0,0h24v24H0V0z" fill="none"/></g><g><path d="M6,19c0,1.1,0.9,2,2,2h8c1.1,0,2-0.9,2-2V7H6V19z M9.17,12.59c-0.39-0.39-0.39-1.02,0-1.41c0.39-0.39,1.02-0.39,1.41,0 L12,12.59l1.41-1.41c0.39-0.39,1.02-0.39,1.41,0s0.39,1.02,0,1.41L13.41,14l1.41,1.41c0.39,0.39,0.39,1.02,0,1.41 s-1.02,0.39-1.41,0L12,15.41l-1.41,1.41c-0.39,0.39-1.02,0.39-1.41,0c-0.39-0.39-0.39-1.02,0-1.41L10.59,14L9.17,12.59z M18,4h-2.5 l-0.71-0.71C14.61,3.11,14.35,3,14.09,3H9.91c-0.26,0-0.52,0.11-0.7,0.29L8.5,4H6C5.45,4,5,4.45,5,5s0.45,1,1,1h12 c0.55,0,1-0.45,1-1S18.55,4,18,4z"/></g>
            </svg>
          </button>
          <div>
            {#if !(temptext.toString().trim().length <= 0 || (temptext === text && (temp_tags.length === tags.length && tags.every((t) => temp_tags.includes(t)))))}
              <button class="action-button" on:click={saveChanges}>
                <svg class="save-icon" xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000"><path d="M0 0h24v24H0V0z" fill="none"/>
                  <path d="M17.59 3.59c-.38-.38-.89-.59-1.42-.59H5c-1.11 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V7.83c0-.53-.21-1.04-.59-1.41l-2.82-2.83zM12 19c-1.66 0-3-1.34-3-3s1.34-3 3-3 3 1.34 3 3-1.34 3-3 3zm1-10H7c-1.1 0-2-.9-2-2s.9-2 2-2h6c1.1 0 2 .9 2 2s-.9 2-2 2z"/>
                </svg>
              </button>
            {/if}
            <button class="action-button" on:click={closePopup}>
              <svg class="exit-icon" xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000">
                <path d="M0 0h24v24H0V0z" fill="none"/><path d="M18.3 5.71c-.39-.39-1.02-.39-1.41 0L12 10.59 7.11 5.7c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41L10.59 12 5.7 16.89c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0L12 13.41l4.89 4.89c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L13.41 12l4.89-4.89c.38-.38.38-1.02 0-1.4z"/>
              </svg>
            </button>
          </div>
        </div>
        <!-- svelte-ignore a11y-autofocus -->
        <textarea style="width: -webkit-fill-available;" placeholder="Jots can't be blank..." autofocus bind:value={temptext}></textarea>
        {#each temp_tags as tag, i}
          <button on:click={() => {
            remove_tag(tag.title);
            search_tags();
          }} class="tag remove-tag">
            <p>{tag.title}</p>
            <svg class="remove-icon" xmlns="http://www.w3.org/2000/svg" height="16px" viewBox="0 0 24 24" width="16px" fill="#000000">
              <path d="M0 0h24v24H0V0z" fill="none"/><path d="M18.3 5.71c-.39-.39-1.02-.39-1.41 0L12 10.59 7.11 5.7c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41L10.59 12 5.7 16.89c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0L12 13.41l4.89 4.89c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L13.41 12l4.89-4.89c.38-.38.38-1.02 0-1.4z"/>
            </svg>
          </button>
        {/each}
        <input style="width: -webkit-fill-available; margin-bottom: 0.5em; margin-top: 1em;" inputmode="search" on:keyup={search_tags} placeholder="Search or Create Tags..." bind:value={query}/>
        {#await search_tags()}
          <p>Loading...</p>
        {:then}
         {#if !tags_list.map(tag => tag.title).includes(query)
          && !temp_tags.map(tag => tag.title).includes(query)
          && query.trim().length > 2 && query.trim().length < 42}
            <div class="left-row">
              <button on:click={add_new_tag_to_jot} class="tag apply-tag">
                <p>{query}</p>
                <svg class="apply-icon" xmlns="http://www.w3.org/2000/svg" height="16px" viewBox="0 0 24 24" width="16px" fill="#000000">
                  <path d="M0 0h24v24H0V0z" fill="none"/><path d="M18 13h-5v5c0 .55-.45 1-1 1s-1-.45-1-1v-5H6c-.55 0-1-.45-1-1s.45-1 1-1h5V6c0-.55.45-1 1-1s1 .45 1 1v5h5c.55 0 1 .45 1 1s-.45 1-1 1z"/>
                </svg>
              </button>
            </div>
            
         {/if}
          
          {#each tags_list as tag, i}
            <button on:click={() => {
                add_tag(tag);
                search_tags();
            }} class="tag apply-tag">
              <p>{tag.title}</p>
              <svg class="apply-icon" xmlns="http://www.w3.org/2000/svg" height="16px" viewBox="0 0 24 24" width="16px" fill="#000000">
                <path d="M0 0h24v24H0V0z" fill="none"/><path d="M18 13h-5v5c0 .55-.45 1-1 1s-1-.45-1-1v-5H6c-.55 0-1-.45-1-1s.45-1 1-1h5V6c0-.55.45-1 1-1s1 .45 1 1v5h5c.55 0 1 .45 1 1s-.45 1-1 1z"/>
              </svg>
            </button>
          {/each}
        {:catch error}
          <p>Error: {error.message}</p>
        {/await}
      </div>
      
    </div>
  {/if}
  
  <style>
  
  </style>