import { M as copy_payload, N as assign_payload, D as pop, z as push, O as head } from "../../chunks/index.js";
import { N as NoteEditor } from "../../chunks/NoteEditor.js";
import { t as triggerRefresh } from "../../chunks/Icon.js";
function _page($$payload, $$props) {
  push();
  let content = "";
  let notePath = "README.md";
  let successMessage = null;
  async function handleSave(newContent) {
    try {
      const response = await fetch(`/api/notes/${notePath}`, {
        method: "PUT",
        headers: { "Content-Type": "text/plain" },
        body: newContent
      });
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      successMessage = "Note saved successfully!";
      setTimeout(() => successMessage = null, 3e3);
      triggerRefresh();
    } catch (error) {
      console.error("Error saving note:", error);
      successMessage = "Failed to save note.";
      setTimeout(() => successMessage = null, 3e3);
    }
  }
  function handleNoteDeleted() {
    triggerRefresh();
    content = "";
    notePath = "README.md";
    successMessage = "Note deleted successfully!";
    setTimeout(() => successMessage = null, 3e3);
  }
  let $$settled = true;
  let $$inner_payload;
  function $$render_inner($$payload2) {
    head($$payload2, ($$payload3) => {
      $$payload3.title = `<title>Note App</title>`;
    });
    NoteEditor($$payload2, {
      notePath,
      onSave: handleSave,
      onDelete: handleNoteDeleted,
      successMessage,
      get content() {
        return content;
      },
      set content($$value) {
        content = $$value;
        $$settled = false;
      }
    });
  }
  do {
    $$settled = true;
    $$inner_payload = copy_payload($$payload);
    $$render_inner($$inner_payload);
  } while (!$$settled);
  assign_payload($$payload, $$inner_payload);
  pop();
}
export {
  _page as default
};
