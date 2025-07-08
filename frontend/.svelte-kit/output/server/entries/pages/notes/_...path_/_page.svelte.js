import { J as getContext, F as store_get, M as copy_payload, N as assign_payload, I as unsubscribe_stores, D as pop, z as push, K as escape_html } from "../../../../chunks/index.js";
import { g as goto } from "../../../../chunks/client.js";
import { N as NoteEditor } from "../../../../chunks/NoteEditor.js";
import { b as backendUrl, t as triggerRefresh } from "../../../../chunks/Icon.js";
const getStores = () => {
  const stores$1 = getContext("__svelte__");
  return {
    /** @type {typeof page} */
    page: {
      subscribe: stores$1.page.subscribe
    },
    /** @type {typeof navigating} */
    navigating: {
      subscribe: stores$1.navigating.subscribe
    },
    /** @type {typeof updated} */
    updated: stores$1.updated
  };
};
const page = {
  subscribe(fn) {
    const store = getStores().page;
    return store.subscribe(fn);
  }
};
function _page($$payload, $$props) {
  push();
  var $$store_subs;
  let noteContent = null;
  let notePathForSave = "";
  let currentUrlPath = "";
  let errorMessage = null;
  let successMessage = null;
  let loading = true;
  let isCreating = false;
  function getParentPath(fullPath) {
    const parts = fullPath.split("/");
    const lastPart = parts[parts.length - 1];
    if (lastPart.includes(".") || lastPart === "README.md") {
      return parts.slice(0, -1).join("/");
    }
    return fullPath;
  }
  async function fetchNoteContent(path) {
    loading = true;
    errorMessage = null;
    noteContent = null;
    isCreating = false;
    try {
      const response = await fetch(`${store_get($$store_subs ??= {}, "$backendUrl", backendUrl)}/api/notes/${path}`);
      if (response.status === 404) {
        noteContent = "";
        const parentPath = getParentPath(path);
        notePathForSave = `${parentPath}/README.md`;
        isCreating = true;
      } else if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      } else {
        const note = await response.json();
        noteContent = note.content;
        if (!path.includes(".")) {
          notePathForSave = `${path}/README.md`;
        } else {
          notePathForSave = path;
        }
      }
    } catch (error) {
      errorMessage = `Failed to fetch note content: ${error.message}`;
      console.error(errorMessage);
      noteContent = null;
    } finally {
      loading = false;
    }
  }
  async function handleSave(content) {
    errorMessage = null;
    successMessage = null;
    try {
      const url = isCreating ? `${store_get($$store_subs ??= {}, "$backendUrl", backendUrl)}/api/notes` : `${store_get($$store_subs ??= {}, "$backendUrl", backendUrl)}/api/notes/${notePathForSave}`;
      const method = isCreating ? "POST" : "PUT";
      const body = isCreating ? JSON.stringify({ path: notePathForSave, content }) : JSON.stringify({ content });
      const response = await fetch(url, {
        method,
        headers: { "Content-Type": "application/json" },
        body
      });
      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`HTTP error! status: ${response.status} - ${errorText}`);
      }
      successMessage = `Note ${isCreating ? "created" : "updated"} successfully!`;
      triggerRefresh();
      if (isCreating) {
        goto(`/notes/${notePathForSave}`);
      }
    } catch (error) {
      errorMessage = `Failed to save note: ${error.message}`;
      console.error(errorMessage);
    }
  }
  async function handleDelete() {
    if (!confirm(`Are you sure you want to delete ${notePathForSave}?`)) {
      return;
    }
    errorMessage = null;
    successMessage = null;
    try {
      const response = await fetch(`${store_get($$store_subs ??= {}, "$backendUrl", backendUrl)}/api/notes/${notePathForSave}`, { method: "DELETE" });
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      successMessage = "Note deleted successfully!";
      triggerRefresh();
      goto("/");
    } catch (error) {
      errorMessage = `Failed to delete note: ${error.message}`;
      console.error(errorMessage);
    }
  }
  if (typeof window !== "undefined" && store_get($$store_subs ??= {}, "$page", page).params.path && store_get($$store_subs ??= {}, "$page", page).params.path !== currentUrlPath) {
    currentUrlPath = store_get($$store_subs ??= {}, "$page", page).params.path;
    fetchNoteContent(currentUrlPath);
  }
  let $$settled = true;
  let $$inner_payload;
  function $$render_inner($$payload2) {
    $$payload2.out += `<div class="note-page svelte-7ionb7">`;
    if (loading) {
      $$payload2.out += "<!--[-->";
      $$payload2.out += `<p>Loading note...</p>`;
    } else if (errorMessage) {
      $$payload2.out += "<!--[1-->";
      $$payload2.out += `<div class="error-message svelte-7ionb7"><p class="svelte-7ionb7">${escape_html(errorMessage)}</p></div>`;
    } else if (noteContent !== null) {
      $$payload2.out += "<!--[2-->";
      NoteEditor($$payload2, {
        notePath: notePathForSave,
        onSave: handleSave,
        onDelete: handleDelete,
        successMessage,
        isCreating,
        get content() {
          return noteContent;
        },
        set content($$value) {
          noteContent = $$value;
          $$settled = false;
        }
      });
    } else {
      $$payload2.out += "<!--[!-->";
      $$payload2.out += `<div class="welcome-message svelte-7ionb7"><h2>Welcome to Notes</h2> <p>Select a note from the list to view or edit it.</p></div>`;
    }
    $$payload2.out += `<!--]--></div>`;
  }
  do {
    $$settled = true;
    $$inner_payload = copy_payload($$payload);
    $$render_inner($$inner_payload);
  } while (!$$settled);
  assign_payload($$payload, $$inner_payload);
  if ($$store_subs) unsubscribe_stores($$store_subs);
  pop();
}
export {
  _page as default
};
