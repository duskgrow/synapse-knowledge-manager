import { invoke } from '@tauri-apps/api/core';
import './style.css';

// Types matching Rust backend
interface Note {
  id: string;
  title: string;
  content_path: string;
  created_at: number;
  updated_at: number;
  word_count: number;
  is_deleted: boolean;
  deleted_at: number | null;
}

interface NoteWithContent {
  note: Note;
  content: string;
}

// App state
let notes: Note[] = [];
let currentNote: NoteWithContent | null = null;
let isNewNote = false;

const el = {
  sidebar: () => document.getElementById('sidebar')!,
  noteList: () => document.getElementById('note-list')!,
  main: () => document.getElementById('main')!,
  titleInput: () => document.getElementById('title-input') as HTMLInputElement,
  contentArea: () => document.getElementById('content-area') as HTMLTextAreaElement,
  status: () => document.getElementById('status')!,
  btnNew: () => document.getElementById('btn-new')!,
  btnSave: () => document.getElementById('btn-save')!,
  btnDelete: () => document.getElementById('btn-delete')!,
};

async function listNotes(): Promise<Note[]> {
  const json = await invoke<string>('list_notes', { include_deleted: false });
  return JSON.parse(json) as Note[];
}

async function getNote(id: string): Promise<NoteWithContent> {
  const json = await invoke<string>('get_note', { id });
  return JSON.parse(json) as NoteWithContent;
}

async function createNote(title: string, content: string): Promise<Note> {
  const json = await invoke<string>('create_note', { title, content });
  return JSON.parse(json) as Note;
}

async function updateNote(id: string, title?: string, content?: string): Promise<void> {
  await invoke('update_note', {
    id,
    title: title !== undefined && title !== '' ? title : null,
    content: content !== undefined ? content : null,
  });
}

async function deleteNote(id: string): Promise<void> {
  await invoke('delete_note', { id });
}

function setStatus(msg: string, isError = false) {
  const s = el.status();
  s.textContent = msg;
  s.style.color = isError ? '#c00' : '#666';
}

function renderNoteList() {
  const ul = el.noteList();
  ul.innerHTML = '';
  for (const n of notes) {
    const li = document.createElement('li');
    li.className = 'note-item' + (currentNote?.note.id === n.id ? ' active' : '');
    li.dataset.id = n.id;
    li.innerHTML = `<span class="note-title">${escapeHtml(n.title || 'Untitled')}</span>`;
    li.addEventListener('click', () => selectNote(n.id));
    ul.appendChild(li);
  }
}

function escapeHtml(s: string): string {
  const div = document.createElement('div');
  div.textContent = s;
  return div.innerHTML;
}

async function selectNote(id: string) {
  try {
    setStatus('Loading…');
    currentNote = await getNote(id);
    isNewNote = false;
    el.titleInput().value = currentNote.note.title;
    el.contentArea().value = currentNote.content;
    el.main().classList.remove('empty');
    el.btnDelete().style.visibility = 'visible';
    renderNoteList();
    setStatus('Ready');
  } catch (e) {
    setStatus('Error: ' + String(e), true);
  }
}

function clearEditor() {
  currentNote = null;
  isNewNote = true;
  el.titleInput().value = '';
  el.contentArea().value = '';
  el.main().classList.add('empty');
  el.btnDelete().style.visibility = 'hidden';
  renderNoteList();
  setStatus('New note');
}

async function saveNote() {
  const title = el.titleInput().value.trim();
  const content = el.contentArea().value;

  try {
    setStatus('Saving…');
    if (isNewNote) {
      const created = await createNote(title || 'Untitled', content);
      notes = await listNotes();
      await selectNote(created.id);
      setStatus('Created');
    } else if (currentNote) {
      await updateNote(currentNote.note.id, title || undefined, content);
      currentNote = { note: { ...currentNote.note, title: title || 'Untitled' }, content };
      notes = await listNotes();
      renderNoteList();
      setStatus('Saved');
    }
  } catch (e) {
    setStatus('Error: ' + String(e), true);
  }
}

async function deleteCurrentNote() {
  if (!currentNote || isNewNote) return;
  if (!confirm('Delete this note?')) return;
  try {
    setStatus('Deleting…');
    await deleteNote(currentNote.note.id);
    notes = await listNotes();
    clearEditor();
    setStatus('Deleted');
  } catch (e) {
    setStatus('Error: ' + String(e), true);
  }
}

function buildUI() {
  const app = document.getElementById('app')!;
  app.innerHTML = `
    <header class="header">
      <h1>Synapse</h1>
      <div id="status" class="status">Ready</div>
    </header>
    <div class="layout">
      <aside id="sidebar" class="sidebar">
        <button id="btn-new" class="btn btn-primary">+ New note</button>
        <ul id="note-list" class="note-list"></ul>
      </aside>
      <main id="main" class="main empty">
        <div class="editor">
          <input id="title-input" type="text" class="title-input" placeholder="Title" />
          <textarea id="content-area" class="content-area" placeholder="Write in Markdown…"></textarea>
          <div class="toolbar">
            <button id="btn-save" class="btn btn-primary">Save</button>
            <button id="btn-delete" class="btn btn-danger" style="visibility: hidden;">Delete</button>
          </div>
        </div>
      </main>
    </div>
  `;

  el.btnNew().addEventListener('click', clearEditor);
  el.btnSave().addEventListener('click', saveNote);
  el.btnDelete().addEventListener('click', deleteCurrentNote);
}

async function init() {
  buildUI();
  try {
    setStatus('Loading…');
    notes = await listNotes();
    renderNoteList();
    if (notes.length > 0) {
      await selectNote(notes[0].id);
    } else {
      clearEditor();
    }
  } catch (e) {
    setStatus('Error: ' + String(e), true);
    notes = [];
    renderNoteList();
    clearEditor();
  }
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', init);
} else {
  init();
}
