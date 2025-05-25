MathMageeks Svelte + Tauri App

## Descripción general

Esta aplicación ofrece una interfaz interactiva para explorar expresiones matemáticas y lógicas. Los usuarios escriben una expresión en ASCII-math o estilo LaTeX en el cuadro de búsqueda, ven una vista previa en vivo con KaTeX y luego la envían a una API remota de “Mathemageeks”. La API devuelve un objeto de “intuición” que describe detalladamente:

* El nombre del concepto
* Su expresión TeX canónica
* Si es un axioma (`es_axioma`)
* Cómo se demuestra (`se_demuestra_con`)
* Expresiones equivalentes (`es_equivalente_a`)
* Casos particulares (`es_caso_particular_de`)
* Instancias (`es_instancia_de`)
* Métodos de resolución (`se_resuelve_con`)
* Aplicaciones (`tiene_aplicacion_en`)

Los resultados se muestran como una cuadrícula de tarjetas; cada tarjeta muestra el nombre y la expresión del concepto, con un botón “Demostrar” (si no es un axioma) que permite profundizar un nivel adicional. Una ruta de migas de pan en el pie de página facilita la navegación hacia atrás a través del árbol de demostración.

## Funcionalidades

* **Vista previa en vivo con KaTeX**
  Mientras escribes, `AsciiMathParser` convierte tu expresión a TeX, y `katex.renderToString` la renderiza inmediatamente.
* **Renderizado automático de resultados**
  Al llegar los datos, `renderMathInElement` aplica KaTeX a cada expresión insertada dinámicamente.
* **Navegación de demostraciones por niveles**
  Haz clic en “Demostrar” para explorar las pruebas de un concepto; el estado se gestiona con `expressionTree` y `levelNames`.
* **Navegación con migas de pan**
  Haz clic en cualquier nivel para volver a ese punto del árbol de demostración.
* **Placeholders de carga**
  Mientras esperas la respuesta de la API, cuatro componentes `<Placeholder />` muestran un esqueleto de interfaz.
* **Autenticación con token Bearer**
  Lee un token `email` de `window.localStorage` y lo envía en la cabecera `Authorization`.

## Stack tecnológico

* **Frontend**

  * SvelteKit
  * TypeScript
  * [@lucide/svelte](https://github.com/lucide-icons/lucide) (íconos)
  * [KaTeX](https://katex.org/) (renderizado de matemáticas)
  * [asciimath2tex](https://github.com/josdejong/asciimath2tex)
* **Backend (Tauri)**

  * Rust + Cargo
  * Webview gestionado por Tauri
* **API**

  * Alojada en `mathemageeks-api-730571690390.northamerica-south1.run.app`
  * Recibe POST JSON `{ expressions: [<cadena TeX>] }`
  * Responde `{ response: <intuitionObject> }`

## Instalación

### Prerrequisitos

1. [Node.js](https://nodejs.org/) ≥ 16
2. [pnpm](https://pnpm.io/) o npm/yarn
3. [Rust](https://www.rust-lang.org/tools/install) y prerrequisitos de [Tauri](https://tauri.app/v1/guides/getting-started/prerequisites)

### Configuración

```bash
git clone https://github.com/your-org/mathmageeks-tauri.git
cd mathmageeks-tauri
pnpm install        # o npm install / yarn
```

## Ejecución

### Desarrollo

```bash
pnpm tauri dev
```

Inicia el servidor de desarrollo de Svelte y la vista de Tauri.

### Build de producción

```bash
pnpm tauri build
```

Genera binarios nativos para tu plataforma.

## Uso

1. **Configura tu token API**
   En la consola del navegador o en código:

   ```js
   window.localStorage.setItem('email', '<tu-api-token>');
   ```
2. **Escribe una expresión** matemática o lógica en el cuadro de búsqueda.
3. **Observa la vista previa** en la parte superior.
4. **Envía** el formulario para llamar a la API remota.
5. **Interactúa** con las tarjetas resultantes:

   * Haz clic en **Demostrar** para profundizar.
   * Usa las **migas de pan** para retroceder.

## Estructura del proyecto

```
src/
├─ components/
│  └─ Placeholder.svelte     # tarjeta de esqueleto durante carga
├─ lib/
│  └─ (utilidades compartidas, stores, etc.)
├─ tauri.conf.json           # configuración de Tauri
├─ src-tauri/                # backend en Rust
├─ App.svelte                # componente principal (código mostrado arriba)
└─ main.ts                   # entrada de SvelteKit
```

## Contribución

1. Haz fork del repositorio
2. Crea tu rama de característica
3. Realiza commits
4. Abre un pull request

## Licencia

Este proyecto usa la licencia MIT.
