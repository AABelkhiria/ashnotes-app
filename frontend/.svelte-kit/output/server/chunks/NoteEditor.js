import { z as push, Q as fallback, R as ensure_array_like, E as attr, K as escape_html, S as bind_props, D as pop, T as attr_class } from "./index.js";
import { marked } from "marked";
import { I as Icon, h as html } from "./Icon.js";
function NoteEditor($$payload, $$props) {
  push();
  let cleanPath, pathSegments;
  let content = $$props["content"];
  let notePath = $$props["notePath"];
  let onSave = $$props["onSave"];
  let onDelete = $$props["onDelete"];
  let successMessage = $$props["successMessage"];
  let isCreating = fallback($$props["isCreating"], false);
  let showSuccessMessage = false;
  function getPath(index) {
    return `/notes/${pathSegments.slice(0, index + 1).join("/")}`;
  }
  if (successMessage) {
    showSuccessMessage = true;
    setTimeout(
      () => {
        successMessage = null;
        showSuccessMessage = false;
      },
      5e3
    );
  }
  cleanPath = notePath.endsWith("/README.md") ? notePath.slice(0, -"/README.md".length) : notePath === "README.md" ? "" : notePath;
  pathSegments = cleanPath.split("/").filter(Boolean);
  const each_array = ensure_array_like(pathSegments);
  $$payload.out += `<div class="note-editor svelte-3p5juq"><div class="note-header svelte-3p5juq"><div class="title-container svelte-3p5juq"><button class="icon-button svelte-3p5juq">`;
  Icon($$payload, { name: "eye" });
  $$payload.out += `<!----></button> <h2 class="svelte-3p5juq"><a href="/notes/README.md" class="svelte-3p5juq">/</a> <!--[-->`;
  for (let i = 0, $$length = each_array.length; i < $$length; i++) {
    let segment = each_array[i];
    $$payload.out += `<a${attr("href", getPath(i))} class="svelte-3p5juq">${escape_html(segment)}</a> `;
    if (i < pathSegments.length - 1) {
      $$payload.out += "<!--[-->";
      $$payload.out += `<span class="svelte-3p5juq">/</span>`;
    } else {
      $$payload.out += "<!--[!-->";
    }
    $$payload.out += `<!--]-->`;
  }
  $$payload.out += `<!--]--></h2></div> <div class="note-actions svelte-3p5juq"><button class="svelte-3p5juq">Save Note</button> `;
  if (!isCreating) {
    $$payload.out += "<!--[-->";
    $$payload.out += `<button class="delete-button svelte-3p5juq">Delete Note</button>`;
  } else {
    $$payload.out += "<!--[!-->";
  }
  $$payload.out += `<!--]--></div></div> `;
  {
    $$payload.out += "<!--[!-->";
    $$payload.out += `<div class="markdown-preview svelte-3p5juq">${html(marked(content))}</div>`;
  }
  $$payload.out += `<!--]--> `;
  if (showSuccessMessage) {
    $$payload.out += "<!--[-->";
    $$payload.out += `<p${attr_class("success-message-centered svelte-3p5juq", void 0, { "hidden": !showSuccessMessage })}>${escape_html(successMessage)}</p>`;
  } else {
    $$payload.out += "<!--[!-->";
  }
  $$payload.out += `<!--]--></div>`;
  bind_props($$props, {
    content,
    notePath,
    onSave,
    onDelete,
    successMessage,
    isCreating
  });
  pop();
}
export {
  NoteEditor as N
};
