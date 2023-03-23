<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { focusTrap } from 'svelte-focus-trap';
    import { onMount } from 'svelte';
    import Dropdown from "$lib/Dropdown.svelte";
    
    export let visible = false;
    
    function closePopup() {
      visible = false;
      document.body.style.overflow = 'auto';
    }
    function handleEscDown(event:  KeyboardEvent) {
      if (visible && event.key === 'Escape') {
        closePopup();
      }
    }
    let output = "";
    let themes: string[] = [];
    
    let selectedTheme = "";
    async function getThemes() {
      selectedTheme = JSON.parse(await invoke("get_settings")).theme;
      themes = await invoke("fetch_all_themes");
    }

    const root = document.documentElement;
    async function changeTheme(theme: string) {
        selectedTheme = theme;
        applyTheme();
        await invoke("set_setting", {key: "theme", val: selectedTheme});
    }
    async function applyTheme() {
      let theme_data: JSON = JSON.parse(await invoke("get_theme", {theme: selectedTheme}));
      if(theme_data.primary_color != undefined) root.style.setProperty('--primary-color', theme_data.primary_color);
      if(theme_data.secondary_color != undefined) root.style.setProperty('--secondary-color', theme_data.secondary_color);
      if(theme_data.negative_color != undefined) root.style.setProperty('--negative-color', theme_data.negative_color);
      if(theme_data.backround_color != undefined) root.style.setProperty('--backround-color', theme_data.backround_color);
      if(theme_data.backround_color_dark != undefined) root.style.setProperty('--backround-color-dark', theme_data.backround_color_dark);
      if(theme_data.backround_color_trans != undefined) root.style.setProperty('--backround-color-trans', theme_data.backround_color_trans);
      if(theme_data.backround_highlight != undefined) root.style.setProperty('--backround-highlight', theme_data.backround_highlight);
      if(theme_data.foreground_font_color != undefined) root.style.setProperty('--foreground-font-color', theme_data.foreground_font_color);
      if(theme_data.foreground_font_color_secondary != undefined) root.style.setProperty('--foreground-font-color-secondary', theme_data.foreground_font_color_secondary);
      if(theme_data.highlight_thickness != undefined) root.style.setProperty('--highlight-thickness', theme_data.highlight_thickness);
    }
    document.addEventListener('keydown', handleEscDown);
  </script>
  {#if visible}
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <div class="popup" tabindex="-1" on:mousedown={closePopup}>
      <!-- svelte-ignore a11y-click-events-have-key-events -->
      <div class="popup-content" use:focusTrap on:mousedown|stopPropagation>
        <div class="action-buttons">
          <div></div>
          <button class="action-button" on:click={closePopup}>
            <svg class="exit-icon" xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000">
              <path d="M0 0h24v24H0V0z" fill="none"/><path d="M18.3 5.71c-.39-.39-1.02-.39-1.41 0L12 10.59 7.11 5.7c-.39-.39-1.02-.39-1.41 0-.39.39-.39 1.02 0 1.41L10.59 12 5.7 16.89c-.39.39-.39 1.02 0 1.41.39.39 1.02.39 1.41 0L12 13.41l4.89 4.89c.39.39 1.02.39 1.41 0 .39-.39.39-1.02 0-1.41L13.41 12l4.89-4.89c.38-.38.38-1.02 0-1.4z"/>
            </svg>
          </button>
        </div>
        <div class="row">
          <svg class="settings-icon" xmlns="http://www.w3.org/2000/svg" enable-background="new 0 0 24 24" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000">
            <g><rect fill="none" height="24" width="24"/></g><g><path d="M12,2C6.49,2,2,6.49,2,12s4.49,10,10,10c1.38,0,2.5-1.12,2.5-2.5c0-0.61-0.23-1.2-0.64-1.67c-0.08-0.1-0.13-0.21-0.13-0.33 c0-0.28,0.22-0.5,0.5-0.5H16c3.31,0,6-2.69,6-6C22,6.04,17.51,2,12,2z M17.5,13c-0.83,0-1.5-0.67-1.5-1.5c0-0.83,0.67-1.5,1.5-1.5 s1.5,0.67,1.5,1.5C19,12.33,18.33,13,17.5,13z M14.5,9C13.67,9,13,8.33,13,7.5C13,6.67,13.67,6,14.5,6S16,6.67,16,7.5 C16,8.33,15.33,9,14.5,9z M5,11.5C5,10.67,5.67,10,6.5,10S8,10.67,8,11.5C8,12.33,7.33,13,6.5,13S5,12.33,5,11.5z M11,7.5 C11,8.33,10.33,9,9.5,9S8,8.33,8,7.5C8,6.67,8.67,6,9.5,6S11,6.67,11,7.5z"/></g>
          </svg>
          <p class="settings-category-label">Appearance</p>
        </div>
        <div class="settings-item">
          {#await getThemes()}
            
          {:then}
          <p class="thin-label">Theme: </p>
          <Dropdown bind:options={themes} bind:selectedOption={selectedTheme} onOptionSelect={changeTheme}></Dropdown>
          {/await}
        </div>
        <div class="settings-item">
          <p class="thin-label">Font Size:</p>
          <p>todo</p>
        </div>
        <div class="row">
          <svg class="settings-icon" xmlns="http://www.w3.org/2000/svg" height="24px" viewBox="0 0 24 24" width="24px" fill="#000000"><path d="M0 0h24v24H0z" fill="none"/>
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm1 15h-2v-6h2v6zm0-8h-2V7h2v2z"/>
          </svg>
          <p class="settings-category-label">About</p>
        </div>
        <div class="settings-item">
          <p class="thin-label">Version:</p>
          <p>todo</p>
        </div>
        <div class="settings-item">
          <p class="thin-label">Github:</p>
          <p>todo</p>
        </div>
        <div class="settings-item">
          <p class="thin-label">License:</p>
          <p>todo</p>
        </div>
        
      </div>
    </div>
  {/if}
  
  <style>
  .settings-icon {
      fill: var(--primary-color);
  }
  .settings-category-label {
    font-weight: bold;
    color: var(--primary-color);
    margin: 0em;
  }
  .settings-item {
    padding-right: 1em;
    padding-left: 1em;
    padding-bottom: 1em;
    display: flex;
    justify-content: space-between;
  }
  </style>