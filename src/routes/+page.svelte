<script land="ts">
  import JotBar from "$lib/JotBar.svelte";
  import JotView from "$lib/JotView.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  
  const root = document.documentElement;
  async function initUI() {
      let theme = JSON.parse(await invoke("get_settings")).theme;
      console.log(theme);
      let theme_data = JSON.parse(await invoke("get_theme", {theme: theme}));
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

</script>

{#await initUI()}
  
{:then} 
  <div class="row"></div>
  <JotBar/>
{/await}
