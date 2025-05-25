<script lang="ts">
  import Placeholder from "$lib/components/Placeholder.svelte";
  import { Search } from "@lucide/svelte";
  import katex from "katex";
  import renderMathInElement from "katex/contrib/auto-render";

  import AsciiMathParser from "asciimath2tex";
  import { tick } from "svelte";

  const parser = new AsciiMathParser();

  let container: HTMLDivElement;
  let breadcrumb: HTMLOListElement;

  let currentExpression = $state("");
  let renderedCurrentExpression = $state("");

  let levelNames: string[] = $state([]);
  let currentView: any[] = $state([]);
  let expressionTree: any[] = $state([]);

  let isLoading = $state(false);

  function formatJson(input: any) {
    if (typeof input === "string") {
      const regex = /"([^"]*)"/g;
      return [...input.matchAll(regex)].map((match) => match[1]);
    }
    return input;
  }
  function refreshRender() {
    const tex = parser.parse(currentExpression);
    renderedCurrentExpression = katex.renderToString(tex, {
      displayMode: true,
      throwOnError: false,
    });
  }
  function refreshExpressions() {
    renderMathInElement(container, {
      throwOnError: false,
      delimiters: [{ left: "$", right: "$", display: true }],
    });
  }

  async function setCurrentView(node: any[], levelName: string) {
    currentView = node;
    expressionTree = [...expressionTree, node];
    levelNames = [...levelNames, levelName];

    console.log(levelNames);

    await tick();

    refreshExpressions();
  }

  async function goToLevel(level: number) {
    currentView = expressionTree[level];
    levelNames = levelNames.slice(0, level);
    expressionTree = expressionTree.slice(0, level);
    
    await tick();
    refreshExpressions();
  }

  async function submitExpression(event: Event) {
    event.preventDefault();
    console.log(currentExpression);

    const tex = parser.parse(currentExpression);

    isLoading = true;
    await tick();

    const intuition = await callExpressionApi(tex);

    isLoading = false;

    await tick();

    console.log(intuition);

    currentView = [intuition];

    expressionTree = [[intuition]];

    await tick();
    refreshExpressions();
  }

  async function callExpressionApi(expression: string): Promise<any> {
    console.log("Calling the API");

    const email = window.localStorage.getItem("email");
    const res = await fetch("https://mathemageeks-api-730571690390.northamerica-south1.run.app/get-expression", {
      method: "POST",
      headers: {
        "Authorization": `Bearer ${email}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        expressions: [expression],
      }),
    });

    const data = await res.json();

    return data.response;
  }
</script>

<div
  class="w-full max-w-4xl gap-3 container mx-auto flex flex-col items-center justify-center"
>
  <div>
    <a
      href="#"
      class="card preset-filled-surface-100-900 border-[1px] border-surface-200-800 card-hover divide-surface-200-800 block max-w-md divide-y overflow-hidden"
    >
      <header>
        <img src="/logo.png" class="aspect-[1/1] w-full" alt="banner" />
      </header>
    </a>
  </div>
  <div
    class="mx-auto card preset-filled-surface-100-900 border-[1px] border-surface-200-800 divide-surface-200-800 block divide-y overflow-hidden"
  >
    <header class="text-2xl">
      {@html renderedCurrentExpression}
    </header>
    <article class="space-y-4 p-4">
      <div>
        <h2 class="h6">
          Introduce alguna expresión matemática y obten datos relevantes para
          esa expresión. Conecta conceptos conforme los vas escribiendo.
        </h2>
        <h3 class="h3">Consulta con mathemageeks</h3>
      </div>
      <div>
        <form onsubmit={submitExpression}>
          <div class="input-group grid-cols-[auto_1fr_auto]">
            <div class="ig-cell preset-tonal">
              <Search size={16} />
            </div>
            <input
              class="ig-input"
              onkeyup={refreshRender}
              bind:value={currentExpression}
              type="search"
              placeholder="Introduce alguna expresión matemática o lógica..."
            />
            <button class="ig-btn btn preset-filled-primary-500" type="submit"
              >Submit</button
            >
          </div>
        </form>
      </div>
    </article>
    <footer class="flex items-center justify-between gap-4 p-4">
      <ol bind:this={breadcrumb} class="flex items-center gap-4">
        {#each levelNames as levelName, i}
          <li>
            <button
              onclick={() => goToLevel(i)}
              class="opacity-60 hover:underline"
              >{@html levelName}</button
            >
          </li>
          <li class="opacity-50" aria-hidden>&rsaquo;</li>
        {/each}
      </ol>
    </footer>
  </div>
  {#if isLoading}
    <div class="w-full max-w-4xl grid grid-cols-2 gap-2">
      <Placeholder />
      <Placeholder />
      <Placeholder />
      <Placeholder />
    </div>
  {/if}
  <div class="w-full max-w-4xl" bind:this={container}>
    {#each currentView as node (node.expression)}
      <div class="w-full max-w-4xl grid grid-cols-2 gap-2">
        <div
          class="card preset-filled-surface-100-900 border-[1px] border-surface-200-800 divide-surface-200-800 block max-w-md divide-y overflow-hidden"
        >
          <article class="space-y-4 p-4">
            <div>
              <h5 class="h5">{node.name}</h5>
            </div>
            <div class="flex">
              {node.expression}
            </div>
          </article>

          <footer class="flex items-center justify-between gap-4 p-4">
            {#if !node.es_axioma}
              <button
                onclick={() =>
                  setCurrentView(
                    node.se_demuestra_con,
                    node.name ?? node.expression
                  )}
                type="button"
                class="btn preset-filled-primary-500">Demostrar</button
              >
            {/if}
          </footer>
        </div>
        {#if node.es_equivalente_a}
          <div
            class="card preset-filled-surface-100-900 border-[1px] border-surface-200-800 divide-surface-200-800 block max-w-md divide-y overflow-hidden"
          >
            <article class="space-y-4 p-4">
              <div>
                <h5 class="h5">Es igual a</h5>
              </div>
              <div class="flex flex-col">
                {#each formatJson(node.es_equivalente_a) as equivalence}
                  <div class="flex flex-row">
                    {equivalence}
                  </div>
                {/each}
              </div>
            </article>
          </div>
        {/if}
        {#if node.es_caso_particular_de}
          <div
            class="card preset-filled-surface-100-900 border-[1px] border-surface-200-800 divide-surface-200-800 block max-w-md divide-y overflow-hidden"
          >
            <article class="space-y-4 p-4">
              <div>
                <h5 class="h5">Es un caso particular de</h5>
              </div>
              <div class="flex flex-col">
                {#each formatJson(node.es_caso_particular_de) as particular}
                  <div class="flex flex-row">
                    {particular}
                  </div>
                {/each}
              </div>
            </article>
          </div>
        {/if}
        {#if node.es_instancia_de}
          <div
            class="card preset-filled-surface-100-900 border-[1px] border-surface-200-800 divide-surface-200-800 block max-w-md divide-y overflow-hidden"
          >
            <article class="space-y-4 p-4">
              <div>
                <h5 class="h5">Es una instancia de</h5>
              </div>
              <div class="flex">
                <ul>
                  {#each node.es_instancia_de as instance}
                    <li>{instance}</li>
                  {/each}
                </ul>
              </div>
            </article>
          </div>
        {/if}
        {#if node.se_resuelve_con}
          <div
            class="card preset-filled-surface-100-900 border-[1px] border-surface-200-800 divide-surface-200-800 block max-w-md divide-y overflow-hidden"
          >
            <article class="space-y-4 p-4">
              <div>
                <h5 class="h5">Se resuelve con</h5>
              </div>
              <div class="flex">
                <ul>
                  {#each node.se_resuelve_con as resuelve}
                    <li>{resuelve}</li>
                  {/each}
                </ul>
              </div>
            </article>
          </div>
        {/if}
        {#if node.tiene_aplicacion_en}
          <div
            class="card preset-filled-surface-100-900 border-[1px] border-surface-200-800 divide-surface-200-800 block max-w-md divide-y overflow-hidden"
          >
            <article class="space-y-4 p-4">
              <div>
                <h5 class="h5">Tiene aplicaciones en</h5>
              </div>
              <div class="flex">
                <ul>
                  {#each node.tiene_aplicacion_en as aplicacion}
                    <li>{aplicacion}</li>
                  {/each}
                </ul>
              </div>
            </article>
          </div>
        {/if}
      </div>
      <hr class="hr border-t-2" />
    {/each}
  </div>
</div>
