import { D as pop, z as push, E as attr, F as store_get, G as slot, I as unsubscribe_stores } from "../../chunks/index.js";
import { w as writable } from "../../chunks/index2.js";
import "clsx";
import { I as Icon, b as backendUrl } from "../../chunks/Icon.js";
const theme = writable("light");
function NoteTree($$payload, $$props) {
  push();
  $$payload.out += `<div class="note-tree svelte-va6g4v"><div class="note-tree-header svelte-va6g4v"><h2 class="svelte-va6g4v"><a href="/notes/README.md" class="svelte-va6g4v">Notes</a></h2> <button class="icon-button" title="New Root Note">`;
  Icon($$payload, { name: "plus" });
  $$payload.out += `<!----></button></div> `;
  {
    $$payload.out += "<!--[-->";
    $$payload.out += `<p class="svelte-va6g4v">Loading...</p>`;
  }
  $$payload.out += `<!--]--></div>`;
  pop();
}
function Settings($$payload, $$props) {
  push();
  backendUrl.subscribe((value) => {
  });
  $$payload.out += `<div class="settings-container svelte-1r6ao13"><button class="icon-button">`;
  Icon($$payload, { name: "settings" });
  $$payload.out += `<!----></button> `;
  {
    $$payload.out += "<!--[!-->";
  }
  $$payload.out += `<!--]--></div>`;
  pop();
}
function _layout($$payload, $$props) {
  push();
  var $$store_subs;
  $$payload.out += `<div class="app-container svelte-1dhmdvo"${attr("data-theme", store_get($$store_subs ??= {}, "$theme", theme))}><aside class="sidebar svelte-1dhmdvo">`;
  NoteTree($$payload);
  $$payload.out += `<!----> <div class="settings-container svelte-1dhmdvo">`;
  Settings($$payload);
  $$payload.out += `<!----></div></aside> <main class="content svelte-1dhmdvo"><!---->`;
  slot($$payload, $$props, "default", {});
  $$payload.out += `<!----></main></div>`;
  if ($$store_subs) unsubscribe_stores($$store_subs);
  pop();
}
export {
  _layout as default
};
