const { invoke } = window.__TAURI__;

async function loadBookshelf() {
  const basePath = document.getElementById('base-path').value;
  if (!basePath) return;
  const list = await invoke('list_directory', { directory: basePath });
  const ul = document.getElementById('bookshelf-list');
  ul.innerHTML = '';
  list.forEach((path) => {
    const li = document.createElement('li');
    li.textContent = path.split('/').pop();
    li.dataset.path = path;
    li.addEventListener('click', () => loadManga(path));
    ul.appendChild(li);
  });
}

async function loadManga(dir) {
  const files = await invoke('search_files', { directory: dir, pattern: '.cbz' });
  const ul = document.getElementById('manga-list');
  ul.innerHTML = '';
  files.forEach((path) => {
    const li = document.createElement('li');
    li.textContent = path.split('/').pop();
    li.dataset.path = path;
    ul.appendChild(li);
  });
  document.getElementById('manga-section').classList.remove('hidden');
}

document.getElementById('load-bookshelf').addEventListener('click', loadBookshelf);

