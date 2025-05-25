<script lang="ts">
  import "../app.css";
  import { AppBar } from "@skeletonlabs/skeleton-svelte";
  import ArrowLeft from "@lucide/svelte/icons/arrow-left";
  import Paperclip from "@lucide/svelte/icons/paperclip";
  import Calendar from "@lucide/svelte/icons/calendar";
  import CircleUser from "@lucide/svelte/icons/circle-user";
</script>

<div class="h-screen grid grid-rows-[auto_1fr_auto]">
  <!-- Main -->
  <main class="p-4 space-y-4">
    <slot /> 
  </main>
  <!-- Footer -->
</div>
